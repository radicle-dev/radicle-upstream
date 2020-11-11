import { get, derived, writable, Readable } from "svelte/store";

import * as identity from "../identity";
import * as project from "../project";
import * as remote from "../remote";
import * as urn from "../urn";
import * as validation from "../validation";

const projectStore = remote.createStore<project.Project>();
export const current = projectStore.readable;

const peersStore = remote.createStore<project.Peer[]>();
export const peerSelection: Readable<remote.Data<{
  default: project.User;
  peers: project.User[];
}>> = derived(peersStore, store => {
  if (
    store.status === remote.Status.NotAsked ||
    store.status === remote.Status.Loading
  ) {
    return store;
  }

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
    if (
      selection.status === remote.Status.NotAsked ||
      selection.status === remote.Status.Loading
    ) {
      return null;
    }

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

export const fetch = (projectUrn: urn.Urn): void => {
  projectStore.loading();
  peersStore.reset();
  selectedPeerStore.set(null);

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

export const selectPeer = (peer: project.User): void => {
  const current = get(selectedPeer);

  if (peer.peerId !== current.peerId) {
    const currentProject = get(projectStore);
    selectedPathStore.set(null);
    selectedPeerStore.set(peer);
    fetchRevisions(currentProject.data.urn);
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

export const VALID_PEER_MATCH = /[1-9A-HJ-NP-Za-km-z]{54}/;

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
