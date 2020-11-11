import { derived, get, writable, Readable } from "svelte/store";

import * as error from "../../error";
import * as project from "../../project";
import * as remote from "../../remote";
import * as source from "../../source";
import * as urn from "../../urn";

import { current as projectStore, selectedPeer } from "../project";

interface Params {
  peer: project.User;
  project: project.Project;
  revision: source.Revision;
  path: string;
}

export enum CodeView {
  File = "FILE",
  Root = "ROOT",
  Error = "ERROR",
}

interface Shared {
  lastCommit: source.LastCommit;
}

interface Error {
  kind: CodeView.Error;
  error: error.Error;
}

interface File extends Shared {
  kind: CodeView.File;
  file: source.Blob;
  path: string;
}

interface Root extends Shared {
  kind: CodeView.Root;
  readme: source.Readme | null;
}

type Code = Error | File | Root;

const revisionsStore = remote.createStore<source.Revisions>();
export const revisionSelection: Readable<remote.Data<{
  default: source.Branch;
  branches: source.Branch[];
  tags: source.Tag[];
}>> = derived([projectStore, revisionsStore], ([project, store]) => {
  if (store.status === remote.Status.Success) {
    let defaultBranch = store.data.branches[0];

    if (project.status === remote.Status.Success) {
      const projectDefault = store.data.branches.find(
        (branch: source.Branch) =>
          branch.name === project.data.metadata.defaultBranch
      );

      if (projectDefault) {
        defaultBranch = projectDefault;
      }
    }

    return {
      status: remote.Status.Success,
      data: { ...store.data, default: defaultBranch },
    };
  }

  return store;
});

const selectedRevisionStore = writable<source.Revision | null>(null);
export const selectedRevision = derived(
  [selectedRevisionStore, revisionSelection],
  ([selected, selection]) => {
    if (selected) {
      return selected;
    }

    if (selection.status === remote.Status.Success) {
      return selection.data.default;
    }

    return null;
  }
);

const selectedPathStore = writable<string | null>(null);

export const params: Readable<remote.Data<Params>> = derived(
  [projectStore, selectedPeer, selectedRevision, selectedPathStore],
  ([remoteProject, peer, revision, selectedPath], set) => {
    if (
      remoteProject.status === remote.Status.NotAsked ||
      remoteProject.status === remote.Status.Loading
    ) {
      set(remoteProject);
    }

    // We can't continue meaningfully if either of them are missing.
    if (!peer || !revision) {
      return;
    }

    if (remoteProject.status === remote.Status.Success) {
      const path = selectedPath ? selectedPath : "";
      const project = remoteProject.data;
      const current = get(params);

      if (current.status === remote.Status.Success) {
        const {
          project: { urn: currentUrn },
          peer: { peerId: currentPeerId },
          revision: currentRevision,
          path: currentPath,
        } = current.data;

        if (
          project.urn === currentUrn &&
          peer.peerId === currentPeerId &&
          revision.type === currentRevision.type &&
          (revision as source.Branch | source.Tag).name ===
            (currentRevision as source.Branch | source.Tag).name &&
          path === currentPath
        ) {
          return;
        }
      }

      set({
        status: remote.Status.Success,
        data: {
          project,
          peer,
          revision,
          path: path ? path : "",
        },
      });
    }

    return (): void => {
      set({ status: remote.Status.NotAsked });
    };
  },
  { status: remote.Status.NotAsked } as remote.Data<Params>
);

export const code: Readable<remote.Data<Code>> = derived(
  [params],
  ([remoteParams], set) => {
    if (
      remoteParams.status === remote.Status.NotAsked ||
      remoteParams.status === remote.Status.Loading
    ) {
      set(remoteParams);
    }

    if (remoteParams.status === remote.Status.Success) {
      const {
        data: { project, peer, revision, path },
      } = remoteParams;
      let lastCommit: source.LastCommit;

      if (path === "") {
        source
          .fetchObject(
            source.ObjectType.Tree,
            project.urn,
            peer.peerId,
            "",
            revision
          )
          .then(tree => {
            lastCommit = tree.info.lastCommit;

            return source.fetchReadme(
              project.urn,
              peer.peerId,
              revision,
              tree as source.Tree
            );
          })
          .then(readme => {
            set({
              status: remote.Status.Success,
              data: {
                kind: CodeView.Root,
                lastCommit,
                readme,
              },
            });
          });
      } else {
        source
          .fetchObject(
            source.ObjectType.Blob,
            project.urn,
            peer.peerId,
            path,
            revision
          )
          .then(blob => {
            set({
              status: remote.Status.Success,
              data: {
                kind: CodeView.File,
                lastCommit: blob.info.lastCommit,
                file: blob as source.Blob,
                path,
              },
            });
          })
          .catch(err => {
            set({
              status: remote.Status.Success,
              data: {
                kind: CodeView.Error,
                error: err,
              },
            });
          });
      }
    }

    return (): void => {
      set({ status: remote.Status.NotAsked });
    };
  },
  { status: remote.Status.NotAsked } as remote.Data<Code>
);

const selectedCommitStore = writable<string | null>(null);
export const commit: Readable<remote.Data<source.Commit>> = derived(
  [projectStore, selectedCommitStore],
  ([remoteProject, commit], set) => {
    if (
      remoteProject.status === remote.Status.NotAsked ||
      remoteProject.status === remote.Status.Loading
    ) {
      set(remoteProject);
    }

    if (!commit) {
      return;
    }

    if (remoteProject.status === remote.Status.Success) {
      const { urn: projectUrn } = remoteProject.data;

      source
        .fetchCommit(projectUrn, commit)
        .then(commit => set({ status: remote.Status.Success, data: commit }));
    }

    return () => {
      selectedCommitStore.set(null);
      set({ status: remote.Status.NotAsked });
    };
  },
  { status: remote.Status.NotAsked } as remote.Data<source.Commit>
);

export const commits: Readable<remote.Data<source.CommitsHistory>> = derived(
  [projectStore, selectedPeer, selectedRevision],
  ([remoteProject, peer, revision], set) => {
    if (
      remoteProject.status === remote.Status.NotAsked ||
      remoteProject.status === remote.Status.Loading
    ) {
      set(remoteProject);
    }

    if (!peer || !revision) {
      return;
    }

    if (remoteProject.status === remote.Status.Success) {
      // TODO(xla): Only branches are supported by the underlying endpoint, this should be extended to tags as well.
      if (revision.type !== source.RevisionType.Branch) {
        return;
      }

      const { urn: projectUrn } = remoteProject.data;

      source
        .fetchCommits(projectUrn, peer.peerId, revision)
        .then(commits => set({ status: remote.Status.Success, data: commits }));
    }
  },
  { status: remote.Status.NotAsked } as remote.Data<source.CommitsHistory>
);

const fetchRevisions = (projectUrn: urn.Urn): void => {
  revisionsStore.loading();

  const currentPeer = get(selectedPeer);

  if (!currentPeer) {
    return;
  }

  source
    .fetchRevisions(projectUrn, currentPeer.peerId)
    .then(revisionsStore.success)
    .catch(revisionsStore.error);
};

export const selectCommit = (hash: string): void => {
  selectedCommitStore.set(hash);
};

export const selectPath = (path: string): void => {
  selectedPathStore.set(path);
};

export const selectRevision = (revision: source.Revision): void => {
  const selected = revision as source.Branch | source.Tag;
  const current = get(selectedRevision);

  if (selected.type !== current.type || selected.name !== current.name) {
    selectedRevisionStore.set(revision);
  }
};
