// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Project, User } from "ui/src/project";

import { get } from "svelte/store";

import * as appearance from "ui/src/appearance";
import * as error from "ui/src/error";
import * as mutexExecutor from "ui/src/mutexExecutor";
import * as proxy from "ui/src/proxy";
import * as remote from "ui/src/remote";
import * as source from "ui/src/source";

export enum ViewKind {
  Blob = "BLOB",
  Error = "ERROR",
  Root = "ROOT",
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

export type View = Blob | Error | Root;

export interface Code {
  lastCommit: source.CommitHeader | null;
  path: string;
  view: View;
}

export interface Screen {
  code: Code;
  history: source.GroupedCommitsHistory;
  peer: User;
  project: Project;
  revisions: Array<source.Branch | source.Tag>;
  selectedPath: source.SelectedPath;
  selectedRevision: source.SelectedRevision;
  tree: source.Tree;
}

const screenStore = remote.createStore<Screen>();
export const store = screenStore.readable;

const screenExecutor = mutexExecutor.create();

export async function fetch(project: Project, peer: User): Promise<void> {
  const screen = get(screenStore);
  if (
    screen.status !== remote.Status.Success ||
    screen.data.project.urn !== project.urn
  ) {
    screenStore.loading();
  }

  try {
    const newScreenData = await screenExecutor.run(async abortSignal => {
      const { branches, tags } = await source.fetchRevisions(
        project.urn,
        peer.peerId,
        { abort: abortSignal }
      );
      const revisions = [...branches, ...tags];
      const defaultBranch = branches.find(
        (branch: source.Branch) =>
          branch.name === project.metadata.defaultBranch
      );
      const selectedRevision = defaultBranch || branches[0];
      const [history, tree, view] = await fetchRevisionRootData(
        project.urn,
        peer.peerId,
        selectedRevision,
        abortSignal
      );

      return {
        code: {
          path: "",
          lastCommit: tree.info.lastCommit,
          view,
        },
        history,
        peer,
        project,
        revisions,
        selectedPath: { selected: "", loading: false },
        selectedRevision: {
          selected: selectedRevision,
          loading: false,
        },
        tree,
      } as Screen;
    });

    if (newScreenData) {
      screenStore.success(newScreenData);
    }
  } catch (err: unknown) {
    screenStore.error(error.fromUnknown(err));
  }
}

export async function selectPath(path: string): Promise<void> {
  const screen = get(screenStore);

  if (
    screen.status === remote.Status.Success &&
    screen.data.selectedPath.selected !== path
  ) {
    const { peer, project, selectedRevision, tree } = screen.data;

    screenStore.success({
      ...screen.data,
      selectedPath: {
        selected: path,
        loading: true,
      },
    });

    const result = await screenExecutor.run(
      async (abortSignal): Promise<View> => {
        try {
          if (path === "") {
            const readme = await source.fetchReadme(
              project.urn,
              peer.peerId,
              selectedRevision.selected,
              tree,
              abortSignal
            );
            return {
              kind: ViewKind.Root,
              readme,
            };
          } else {
            const blob = await source.fetchBlob(
              project.urn,
              peer.peerId,
              path,
              selectedRevision.selected,
              get(appearance.theme),
              abortSignal
            );
            return {
              kind: ViewKind.Blob,
              blob,
            };
          }
        } catch (err: unknown) {
          return {
            kind: ViewKind.Error,
            error: error.fromUnknown(err),
          };
        }
      }
    );
    if (result) {
      screenStore.success({
        ...screen.data,
        code: {
          lastCommit: tree.info.lastCommit,
          path,
          view: result,
        },
        selectedPath: {
          selected: path,
          loading: false,
        },
      });
    }
  }
}

export async function selectRevision(
  revision: source.Branch | source.Tag
): Promise<void> {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const { peer, project, selectedRevision: revisionState } = screen.data;

    if (
      revisionState.selected.type === revision.type &&
      revisionState.selected.name === revision.name
    ) {
      return;
    }

    screenStore.success({
      ...screen.data,
      selectedRevision: {
        selected: revision,
        loading: true,
      },
      selectedPath: {
        selected: "",
        loading: true,
      },
    });

    try {
      const result = await screenExecutor.run(abortSignal => {
        return fetchRevisionRootData(
          project.urn,
          peer.peerId,
          revision,
          abortSignal
        );
      });
      if (result) {
        const [history, tree, view] = result;

        screenStore.success({
          ...screen.data,
          code: {
            path: "",
            lastCommit: tree.info.lastCommit,
            view,
          },
          tree,
          history,
          selectedRevision: {
            selected: revision,
            loading: false,
          },
        });
      }
    } catch (err: unknown) {
      screenStore.error(error.fromUnknown(err));
    }
  }
}

async function fetchRevisionRootData(
  projectUrn: string,
  peerId: string,
  revision: source.RevisionSelector,
  abortSignal: AbortSignal
): Promise<[source.GroupedCommitsHistory, source.Tree, View]> {
  const [commits, [tree, view]] = await Promise.all([
    source.fetchCommits(projectUrn, peerId, revision),
    (async () => {
      const tree = await proxy.client.source.treeGet(
        {
          projectUrn,
          peerId,
          revision,
          prefix: "",
        },
        {
          abort: abortSignal,
        }
      );

      let view: View;
      try {
        const readme = await source.fetchReadme(
          projectUrn,
          peerId,
          revision,
          tree,
          abortSignal
        );
        view = {
          kind: ViewKind.Root,
          readme,
        };
      } catch (err: unknown) {
        view = {
          kind: ViewKind.Error,
          error: error.fromUnknown(err),
        };
      }
      return [tree, view];
    })(),
  ]);

  const groupedHistory = source.groupCommitHistory(commits);
  return [groupedHistory, tree, view];
}
