import { get, derived, writable, Readable } from "svelte/store";

import * as error from "../error";
import * as identity from "../identity";
import * as project from "../project";
import * as remote from "../remote";
import * as source from "../source";
import * as urn from "../urn";
import * as validation from "../validation";

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

const projectStore = remote.createStore<project.Project>();
export const current = projectStore.readable;

const peersStore = remote.createStore<project.Peer[]>();
export const peerSelection: Readable<remote.Data<{
  default: project.User;
  peers: project.User[];
}>> = derived(peersStore, store => {
  if (store.status === remote.Status.Success) {
    const peers = store.data
      .filter(
        peer => peer.status.type === project.ReplicationStatusType.Replicated
      )
      .map(peer => {
        const { role, user } = peer.status as project.Replicated;
        return { type: peer.type, peerId: peer.peerId, identity: user, role };
      });

    // TODO(xla): Apply proper heuristic to set default.
    return {
      status: remote.Status.Success,
      data: { default: peers[0], peers },
    };
  }

  return store;
});

const selectedPeerStore = writable<project.User | null>(null);
export const selectedPeer = derived(
  [selectedPeerStore, peerSelection],
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

export const pendingPeers: Readable<remote.Data<{
  peers: project.Peer[];
}>> = derived(peersStore, store => {
  if (store.status === remote.Status.Success) {
    const peers = store.data.filter(
      peer => peer.status.type === project.ReplicationStatusType.NotReplicated
    );

    return {
      status: remote.Status.Success,
      data: { peers },
    };
  }

  return store;
});

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
          });
      }
    }
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
  },
  { status: remote.Status.NotAsked } as remote.Data<source.Commit>
);

export const commits: Readable<remote.Data<source.CommitsHistory>> = derived(
  [projectStore, selectedPeerStore, selectedRevisionStore],
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

export const fetch = (projectUrn: urn.Urn): void => {
  projectStore.loading();
  peersStore.reset();
  revisionsStore.reset();

  project
    .fetch(projectUrn)
    .then(p => {
      projectStore.success(p);
      fetchPeers(projectUrn);
    })
    .catch(projectStore.error);
};

const fetchPeers = (projectUrn: urn.Urn): void => {
  peersStore.loading();

  project
    .fetchPeers(projectUrn)
    .then(peers => {
      peersStore.success(peers);
      fetchRevisions(projectUrn);
    })
    .catch(peersStore.error);
};

const fetchRevisions = (projectUrn: urn.Urn): void => {
  revisionsStore.loading();

  const currentPeer = get(selectedPeer);

  if (!currentPeer) {
    console.log("Can't fetch revisions without selected peer");
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

export const selectPeer = (peer: project.User): void => {
  const current = get(selectedPeer);

  if (peer.peerId !== current.peerId) {
    const currentProject = get(projectStore);
    selectedPeerStore.set(peer);
    fetchRevisions(currentProject.data.urn);
  }
};

export const selectRevision = (revision: source.Revision): void => {
  const selected = revision as source.Branch | source.Tag;
  const current = get(selectedRevision);

  if (selected.type !== current.type || selected.name !== current.name) {
    selectedRevisionStore.set(revision);
  }
};

export const trackPeer = (
  projectUrn: urn.Urn,
  peerId: identity.PeerId
): void => {
  project
    .trackPeer(projectUrn, peerId)
    .then(() => fetchPeers(projectUrn))
    .catch(peersStore.error);
};

export const untrackPeer = (
  projectUrn: urn.Urn,
  peerId: identity.PeerId
): void => {
  project
    .untrackPeer(projectUrn, peerId)
    .then(() => fetchPeers(projectUrn))
    .catch(peersStore.error);
};

const VALID_PEER_MATCH = /[1-9A-HJ-NP-Za-km-z]{54}/;

const checkPeerUniqueness = (peer: string): Promise<boolean> => {
  return Promise.resolve(
    !get(peersStore)
      .data.map((peer: project.Peer) => {
        return peer.peerId;
      })
      .includes(peer)
  );
};

export const peerValidation = validation.createValidationStore(
  {
    format: {
      pattern: VALID_PEER_MATCH,
      message: "This is not a valid remote",
    },
  },
  [
    {
      promise: checkPeerUniqueness,
      validationMessage: "This remote is already being followed",
    },
  ]
);

export const addPeer = async (
  projectId: urn.Urn,
  newRemote: identity.PeerId
): Promise<boolean> => {
  // This has to be awaited contrary to what tslint suggests, because we're
  // running async remote validations in in the background. If we remove the
  // async then the seed input form will have to be submitted twice to take any
  // effect.
  await peerValidation.validate(newRemote);
  if (get(peerValidation).status !== validation.ValidationStatus.Success)
    return false;

  trackPeer(projectId, newRemote);
  return true;
};

export const removePeer = (
  projectId: urn.Urn,
  remote: identity.PeerId
): void => {
  untrackPeer(projectId, remote);
};
