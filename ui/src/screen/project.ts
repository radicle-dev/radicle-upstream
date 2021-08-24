// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { get, derived, Readable } from "svelte/store";

import * as mutexExecutor from "ui/src/mutexExecutor";
import * as error from "../error";
import type { PeerId } from "../identity";
import * as project from "../project";
import * as remote from "../remote";
import type { Urn } from "../urn";
import * as validation from "../validation";
import * as proxy from "../proxy";

interface Screen {
  peers: project.Peer[];
  peerSelection: project.User[];
  project: project.Project;
  selectedPeer: project.User;
}

const refreshExecutor = mutexExecutor.create();

const screenStore = remote.createStore<Screen>();
export const store = screenStore.readable;

export const fetch = (projectUrn: Urn): void => {
  screenStore.loading();

  proxy.client.project
    .get(projectUrn)
    .then(async prj => {
      const peers = await proxy.client.project.listPeers(projectUrn);
      const peerSelection = project.userList(peers);
      throwUnlessPeersPresent(peerSelection, projectUrn);
      screenStore.success({
        peers,
        peerSelection,
        project: prj,
        selectedPeer: peerSelection[0],
      });
    })
    .catch(err => screenStore.error(error.fromUnknown(err)));
};

export const refreshPeers = async (): Promise<void> => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    try {
      const peers = await refreshExecutor.run(abort =>
        proxy.client.project.listPeers(screen.data.project.urn, { abort })
      );
      if (peers === undefined) {
        return;
      }

      const peerSelection = project.userList(peers);
      throwUnlessPeersPresent(peerSelection, screen.data.project.urn);
      screenStore.success({
        ...screen.data,
        peers,
        peerSelection,
      });
    } catch (err: unknown) {
      screenStore.error(error.fromUnknown(err));
    }
  }
};

export const selectPeer = (peer: project.User): void => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const { data: current } = screen;

    if (peer.peerId !== current.selectedPeer.peerId) {
      screenStore.success({ ...current, selectedPeer: peer });
    }
  }
};

export const pendingPeers: Readable<
  remote.Data<{
    peers: project.Peer[];
  }>
> = derived(screenStore, (store): remote.Data<{ peers: project.Peer[] }> => {
  if (store.status === remote.Status.Success) {
    const peers = store.data.peers.filter(
      peer =>
        peer.status.type === project.PeerReplicationStatusType.NotReplicated
    );

    return {
      status: remote.Status.Success,
      data: { peers },
    };
  } else {
    return store;
  }
});

export const trackPeer = (projectUrn: Urn, peerId: PeerId): void => {
  proxy.client.project
    .peerTrack(projectUrn, peerId)
    .then(() => refreshPeers())
    .catch(err => screenStore.error(error.fromUnknown(err)));
};

export const untrackPeer = (projectUrn: Urn, peerId: PeerId): void => {
  proxy.client.project
    .peerUntrack(projectUrn, peerId)
    .then(() => refreshPeers())
    .catch(err => screenStore.error(error.fromUnknown(err)));
};

export const VALID_PEER_MATCH = /[1-9A-HJ-NP-Za-km-z]{54}/;

const checkPeerUniqueness = (peer: string): Promise<boolean> => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const {
      data: { peers },
    } = screen;
    const includes = !peers
      .map((peer: project.Peer) => {
        return peer.peerId;
      })
      .includes(peer);

    return Promise.resolve(includes);
  }

  return Promise.resolve(false);
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
  projectId: Urn,
  newRemote: PeerId
): Promise<boolean> => {
  // This has to be awaited contrary to what tslint suggests, because we're
  // running async remote validations in in the background. If we remove the
  // async then the seed input form will have to be submitted twice to take any
  // effect.
  await peerValidation.validate(newRemote);
  if (get(peerValidation).status !== validation.ValidationStatus.Success) {
    return false;
  }

  trackPeer(projectId, newRemote);
  return true;
};

export const removePeer = (projectId: Urn, peerId: PeerId): void => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const { peerSelection, selectedPeer } = screen.data;

    untrackPeer(projectId, peerId);

    if (selectedPeer.peerId === peerId) {
      screenStore.success({
        ...screen.data,
        selectedPeer: peerSelection[0],
      });
    }
  }
};

const throwUnlessPeersPresent = (peers: project.User[], projectId: Urn) => {
  if (peers.length === 0) {
    throw new Error(`Project ${projectId} is missing peers`);
  }
};
