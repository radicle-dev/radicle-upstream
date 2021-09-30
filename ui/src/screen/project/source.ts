// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { derived, get, writable } from "svelte/store";
import type { Readable, Writable } from "svelte/store";

import * as error from "ui/src/error";
import * as patch from "ui/src/project/patch";
import * as localPeer from "ui/src/localPeer";
import type { Project, User } from "ui/src/project";
import * as remote from "ui/src/remote";
import * as router from "ui/src/router";
import * as source from "ui/src/source";
import * as appearance from "ui/src/appearance";
import * as proxy from "ui/src/proxy";

export enum ViewKind {
  Aborted = "ABORTED",
  Blob = "BLOB",
  Error = "ERROR",
  Root = "ROOT",
}

export interface Aborted {
  kind: ViewKind.Aborted;
}

export interface Blob {
  kind: ViewKind.Blob;
  blob: source.Blob;
}

interface Error {
  kind: ViewKind.Error;
  error: error.Error;
}

export interface Root {
  kind: ViewKind.Root;
  readme: source.Readme | null;
}

export type View = Aborted | Blob | Error | Root;

export interface Code {
  lastCommit: source.CommitHeader | null;
  path: string;
  view: View;
}

export interface Screen {
  code: Writable<Code>;
  history: source.GroupedCommitsHistory;
  patches: patch.Patch[];
  peer: User;
  project: Project;
  revisions: Array<source.Branch | source.Tag>;
  requestInProgress: AbortController | null;
  selectedPath: Readable<source.SelectedPath>;
  selectedRevision: source.SelectedRevision;
  tree: Writable<source.Tree>;
}

const pathStore = writable<source.SelectedPath>({
  request: null,
  selected: "",
});

const screenStore = remote.createStore<Screen>();
export const store = screenStore.readable;

export const fetch = async (project: Project, peer: User): Promise<void> => {
  if (!screenStore.is(remote.Status.Success)) {
    screenStore.loading();
  }

  const fetchTreeRoot = async (
    selectedRevision: source.RevisionSelector
  ): Promise<[source.Tree, Code]> => {
    const tree = await proxy.client.source.treeGet({
      projectUrn: project.urn,
      peerId: peer.peerId,
      revision: selectedRevision,
      prefix: "",
    });
    const root = await fetchCode(project, peer, selectedRevision, tree, "");
    return [tree, root];
  };

  try {
    const { branches, tags } = await source.fetchRevisions(
      project.urn,
      peer.peerId
    );
    const revisions = [...branches, ...tags];
    const patches = await patch.getAll(project.urn);
    const defaultBranch = branches.find(
      (branch: source.Branch) => branch.name === project.metadata.defaultBranch
    );
    const selectedRevision = defaultBranch || branches[0];
    const [history, [tree, root]] = await Promise.all([
      source.fetchCommits(project.urn, peer.peerId, selectedRevision),
      fetchTreeRoot(selectedRevision),
    ]);
    const groupedHistory = source.groupCommitHistory(history);

    screenStore.success({
      code: writable<Code>(root),
      history: groupedHistory,
      patches,
      peer,
      project,
      revisions,
      requestInProgress: null,
      selectedPath: derived(pathStore, store => store),
      selectedRevision: {
        request: null,
        selected: selectedRevision,
      },
      tree: writable<source.Tree>(tree),
    });
  } catch (err: unknown) {
    screenStore.error(error.fromUnknown(err));
  }
};

export const watchPatchUpdates = (): (() => void) => {
  return localPeer.projectEvents.onValue(event => {
    const screen = get(screenStore);
    if (screen.status === remote.Status.Success) {
      const patchUrnPrefix = `${screen.data.project.urn}/tags/${patch.TAG_PREFIX}`;
      const defaultBranchUrnRe = RegExp(
        `${screen.data.project.urn}/heads/${screen.data.project.metadata.defaultBranch}`
      );
      if (
        event.urn.startsWith(patchUrnPrefix) ||
        event.urn.match(defaultBranchUrnRe)
      ) {
        refreshPatches();
      }
    }
  });
};

const refreshPatches = (): void => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const { data: current } = screen;
    const {
      project: { urn },
      requestInProgress,
    } = current;

    if (requestInProgress) {
      requestInProgress.abort();
    }

    const request = new AbortController();
    screenStore.success({
      ...current,
      requestInProgress: request,
    });
    patch
      .getAll(urn)
      .then(patches => {
        screenStore.success({
          ...current,
          patches,
          requestInProgress: null,
        });
      })
      .catch(err => screenStore.error(error.fromUnknown(err)));
  }
};

export const selectPath = async (path: string): Promise<void> => {
  const screen = get(screenStore);
  const current = get(pathStore);

  if (current.selected !== path && screen.status === remote.Status.Success) {
    const {
      data: {
        peer,
        project,
        selectedRevision: { selected: revision },
        tree,
      },
    } = screen;

    const code = await fetchCode(project, peer, revision, get(tree), path);
    if (code.view.kind !== ViewKind.Aborted) {
      screen.data.code.set(code);
    }
  }
};

export const selectRevision = async (
  revision: source.Branch | source.Tag
): Promise<void> => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const {
      data: { code, peer, project, selectedRevision: current, tree },
    } = screen;

    if (
      current.selected.type === revision.type &&
      current.selected.name === revision.name
    ) {
      return;
    }

    if (current.request) {
      current.request.abort();
    }

    const request = new AbortController();
    const fetchTreeCode = async (): Promise<[source.Tree, Code]> => {
      const tree = await proxy.client.source.treeGet(
        {
          projectUrn: project.urn,
          peerId: peer.peerId,
          revision,
          prefix: "",
        },
        {
          abort: request.signal,
        }
      );

      const newCode = await fetchCode(
        project,
        peer,
        revision,
        tree,
        get(pathStore).selected
      );

      return [tree, newCode];
    };

    screenStore.success({
      ...screen.data,
      selectedRevision: {
        request,
        selected: revision,
      },
    });

    try {
      const [history, [newTree, newCode]] = await Promise.all([
        source.fetchCommits(project.urn, peer.peerId, revision),
        fetchTreeCode(),
      ]);
      const groupedHistory = source.groupCommitHistory(history);
      code.set(newCode);
      tree.set(newTree);

      screenStore.success({
        ...screen.data,
        history: groupedHistory,
        selectedRevision: {
          request: null,
          selected: revision,
        },
      });
    } catch (err: unknown) {
      screenStore.error(error.fromUnknown(err));
    }
  }
};

const commitStore = remote.createStore<source.Commit>();
export const commit = commitStore.readable;

export const fetchCommit = async (sha1: string): Promise<void> => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const {
      data: { project },
    } = screen;

    try {
      commitStore.success(
        await proxy.client.source.commitGet({ projectUrn: project.urn, sha1 })
      );
    } catch (err: unknown) {
      const e = error.fromUnknown(err);
      if (e.message.match("object not found")) {
        commitStore.error(e);
      } else {
        error.show(
          new error.Error({
            code: error.Code.CommitFetchFailure,
            message: "Could not fetch commit",
            source: err,
          })
        );
      }
    }
  }
};

export const selectCommit = (commit: source.CommitHeader): void => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const {
      data: { project },
    } = screen;

    router.push({
      type: "project",
      params: {
        urn: project.urn,
        activeView: { type: "commit", commitHash: commit.sha1 },
      },
    });
  }
};

const fetchBlob = async (
  project: Project,
  peer: User,
  revision: source.RevisionSelector,
  path: string,
  signal: AbortSignal
): Promise<Code> => {
  const blob = await source.fetchBlob(
    project.urn,
    peer.peerId,
    path,
    revision,
    get(appearance.theme),
    signal
  );
  return {
    lastCommit: blob.info.lastCommit,
    path,
    view: {
      kind: ViewKind.Blob,
      blob,
    },
  };
};

const fetchCode = async (
  project: Project,
  peer: User,
  revision: source.RevisionSelector,
  tree: source.Tree,
  path: string
): Promise<Code> => {
  const currentPath = get(pathStore);
  if (currentPath.request) {
    currentPath.request.abort();
  }

  const request = new AbortController();
  pathStore.set({ request, selected: path });

  let code: Code;
  try {
    if (path === "") {
      code = await fetchRoot(project, peer, revision, tree, request.signal);
    } else {
      code = await fetchBlob(project, peer, revision, path, request.signal);
    }
  } catch (err: unknown) {
    // An in-flight request was aborted, we wait for the next one to arrive.
    if (err instanceof globalThis.Error && err.name === "AbortError") {
      code = {
        lastCommit: tree.info.lastCommit,
        path,
        view: {
          kind: ViewKind.Aborted,
        },
      };
    } else {
      code = {
        lastCommit: tree.info.lastCommit,
        path,
        view: {
          kind: ViewKind.Error,
          error: error.fromUnknown(err),
        },
      };
    }
  }

  if (code.view.kind !== ViewKind.Aborted) {
    pathStore.set({ request: null, selected: path });
  }

  return code;
};

const fetchRoot = async (
  project: Project,
  peer: User,
  revision: source.RevisionSelector,
  tree: source.Tree,
  signal: AbortSignal
): Promise<Code> => {
  return {
    lastCommit: tree.info.lastCommit,
    path: "",
    view: {
      kind: ViewKind.Root,
      readme: await source.fetchReadme(
        project.urn,
        peer.peerId,
        revision,
        tree,
        signal
      ),
    },
  };
};
