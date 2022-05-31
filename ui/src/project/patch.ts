// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Identity } from "proxy-client/identity";
import type { Project } from "ui/src/project";

import * as ipc from "ui/src/ipc";
import * as notification from "ui/src/notification";
import * as proxy from "ui/src/proxy";
import * as proxyProject from "proxy-client/project";
import * as router from "ui/src/router";
import * as source from "ui/src/source";

import * as zod from "zod";
import { flatten } from "lodash";

export interface Patch {
  id: string;
  peerId: string;
  identity: Identity | null;
  title: string | null;
  description: string | null;
  commit: string;
  mergeBase: string | null;
  merged: boolean;
  status: {
    current: "open" | "merged" | "closed";
    byPeerId?: string;
  };
}

// Global identifier for a patch
export interface PatchId {
  // URN of the project the patch is for
  projectUrn: string;
  // The peer that authored the patch
  peerId: string;
  // The name of the patch given to it by the peer. This corresponds to
  // `Patch.id`.
  name: string;
}

export interface PatchDetails {
  patch: Patch;
  commits: source.GroupedCommitsHistory;
}

// Handle to reference the patch, for example on the command line.
//
// The handle of a patch is `<Peer ID>/<patch name>`.
export function handle(patch: Patch): string {
  return `${patch.peerId}/${patch.id}`;
}

function inferStatus(
  events: proxyProject.EventEnvelope<PatchEventOrUnknown>[],
  proxyPatch: proxyProject.Patch,
  delegates: string[]
): {
  status: "closed" | "open" | "merged";
  byPeerId?: string;
} {
  // Ignore events by users who are neither delegates nor the patch creator
  const lastStatusUpdate = events.find(
    e =>
      (e.peer_id === proxyPatch.peer.peerId || delegates.includes(e.peer_id)) &&
      e.event.type === "setStatus"
  );

  const merged = proxyPatch.mergeBase === proxyPatch.commit;

  if (merged) {
    return {
      status: "merged",
    };
  } else {
    const event = lastStatusUpdate?.event;
    const status =
      (event && event.type === "setStatus" && event.data.status) || "open";

    return {
      status,
      byPeerId: lastStatusUpdate?.peer_id,
    };
  }
}

function makePatch(
  proxyPatch: proxyProject.Patch,
  project: Project,
  patchEvents: proxyProject.EventEnvelope<PatchEventOrUnknown>[]
): Patch {
  const messageLines = proxyPatch.message ? proxyPatch.message.split("\n") : [];
  const title = messageLines.shift() || null;
  // Throw away empty line that separates title from description
  messageLines.shift();
  const description = messageLines.length > 0 ? messageLines.join("\n") : null;
  const identity =
    proxyPatch.peer.status.type === "replicated"
      ? proxyPatch.peer.status.user
      : null;

  const merged = proxyPatch.mergeBase === proxyPatch.commit;

  const { status, byPeerId } = inferStatus(
    patchEvents,
    proxyPatch,
    flatten(Object.values(project.metadata.delegates))
  );

  return {
    id: proxyPatch.id,
    peerId: proxyPatch.peer.peerId,
    identity,
    title,
    description,
    commit: proxyPatch.commit,
    mergeBase: proxyPatch.mergeBase,
    status: {
      current: status,
      byPeerId,
    },
    merged,
  };
}

export const TAG_PREFIX = "radicle-patch/";

export const getAll = async (
  project: Project,
  options?: proxy.RequestOptions
): Promise<Patch[]> => {
  const proxyPatches = await proxy.client.project.patchList(
    project.urn,
    options
  );
  return Promise.all(
    proxyPatches.map(async patch => {
      const patchEvents = await getAllEvents({
        projectUrn: project.urn,
        peerId: patch.peer.peerId,
        name: patch.id,
      });
      return makePatch(patch, project, patchEvents);
    })
  );
};

export async function publishEvent(
  patchId: PatchId,
  event: PatchEvent
): Promise<void> {
  await proxy.client.project.eventPublish(
    patchId.projectUrn,
    eventTopic(patchId),
    event
  );
}

async function getAllEvents(
  patchId: PatchId
): Promise<Array<proxyProject.EventEnvelope<PatchEventOrUnknown>>> {
  const envelopes = await proxy.client.project.eventList(
    patchId.projectUrn,
    eventTopic(patchId)
  );
  return envelopes.map(({ event, ...envelope }) => {
    const patchEvent = patchEventOrUnknownSchema.parse(event);
    return {
      event: patchEvent,
      ...envelope,
    };
  });
}

function eventTopic(patchId: PatchId) {
  return ["patch", patchId.peerId, patchId.name].join("/");
}

export type PatchEvent = {
  type: "setStatus";
  data: { status: "open" | "closed" };
};

export type PatchEventOrUnknown =
  | PatchEvent
  | { type: null; unknownType: string; data?: unknown };

// We can’t have explicit schema type annotations because we’re using
// `.transform()`.
const patchEventOrUnknownSchema = zod.union([
  zod.object({
    type: zod.literal("setStatus"),
    data: zod.object({
      status: zod.enum(["open", "closed"]),
    }),
  }),
  zod
    .object({ type: zod.string(), data: zod.unknown() })
    .transform<{ type: null; unknownType: string; data?: unknown }>(
      ({ type, data }) => ({ type: null, unknownType: type, data })
    ),
]);

export const getDetails = async (
  project: Project,
  peerId: string,
  id: string
): Promise<PatchDetails | undefined> => {
  const patches = await getAll(project);
  const patch = patches.find(patch => {
    return patch.peerId === peerId && patch.id === id;
  });
  if (!patch) {
    return;
  }

  const commits = await getCommits(project, patch);
  return {
    patch,
    commits,
  };
};

// Get the grouped commit history from a patch.
//
// If the head commit of the delegate's default branch (the “base
// head”) is in the patch commit history that commit and all its
// ancestory are filtered.
//
// Note that this is a limited approach and does not filter commits if
// the patch head and the default branch head have a common ancestor
// but the patch head is not a descendent of the default branch head.
//
// If the `patch` is `merged` the filtering is skipped and all commits
// are listed.
const getCommits = async (
  project: Project,
  patch: Patch
): Promise<source.GroupedCommitsHistory> => {
  if (!patch.merged && patch.mergeBase) {
    const patchCommits = await source.fetchCommits(project.urn, patch.peerId, {
      type: source.RevisionType.Sha,
      sha: patch.commit,
    });

    const baseHeadIndex = patchCommits.history.findIndex(
      ch => ch.sha1 === patch.mergeBase
    );
    const filteredPatchCommits = patchCommits.history.slice(
      0,
      baseHeadIndex === -1 ? 0 : baseHeadIndex
    );

    return source.groupCommitHistory({
      history: filteredPatchCommits,
      stats: { ...patchCommits.stats, commits: filteredPatchCommits.length },
    });
  } else {
    const patchCommits = await source.fetchCommits(project.urn, patch.peerId, {
      type: source.RevisionType.Sha,
      sha: patch.commit,
    });
    return source.groupCommitHistory(patchCommits);
  }
};

export function copyPatchUrlToClipboard(projectId: string, patch: Patch): void {
  const patchUrl = router.routeToUri({
    type: "project",
    params: {
      urn: projectId,
      activeView: {
        type: "patch",
        peerId: patch.peerId,
        id: patch.id,
        view: "commits",
      },
    },
  });

  ipc.copyToClipboard(patchUrl);
  notification.show({
    type: "info",
    message: "Shareable link copied to your clipboard",
  });
}
