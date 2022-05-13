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

export interface Patch {
  id: string;
  peerId: string;
  identity: Identity | null;
  title: string | null;
  description: string | null;
  commit: string;
  mergeBase: string | null;
  merged: boolean;
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

function makePatch(proxyPatch: proxyProject.Patch): Patch {
  const messageLines = proxyPatch.message ? proxyPatch.message.split("\n") : [];
  const title = messageLines.shift() || null;
  // Throw away empty line that separates title from description
  messageLines.shift();
  const description = messageLines.length > 0 ? messageLines.join("\n") : null;
  const identity =
    proxyPatch.peer.status.type === "replicated"
      ? proxyPatch.peer.status.user
      : null;

  return {
    id: proxyPatch.id,
    peerId: proxyPatch.peer.peerId,
    identity,
    title,
    description,
    commit: proxyPatch.commit,
    mergeBase: proxyPatch.mergeBase,
    merged: proxyPatch.mergeBase === proxyPatch.commit,
  };
}

export const TAG_PREFIX = "radicle-patch/";

export const getAll = async (
  projectUrn: string,
  options?: proxy.RequestOptions
): Promise<Patch[]> => {
  const proxyPatches = await proxy.client.project.patchList(
    projectUrn,
    options
  );
  return proxyPatches.map(makePatch);
};

export const getDetails = async (
  project: Project,
  peerId: string,
  id: string
): Promise<PatchDetails | undefined> => {
  const patches = await getAll(project.urn);
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
      },
    },
  });

  ipc.copyToClipboard(patchUrl);
  notification.show({
    type: "info",
    message: "Shareable link copied to your clipboard",
  });
}
