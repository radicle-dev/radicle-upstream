// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { PeerId } from "ui/src/identity";

import { get } from "svelte/store";

import * as error from "ui/src/error";
import * as mutexExecutor from "ui/src/mutexExecutor";
import * as project from "ui/src/project";
import * as proxy from "ui/src/proxy";
import * as remote from "ui/src/remote";
import * as validation from "ui/src/validation";

interface Screen {
  peers: project.Peer[];
  peerSelection: project.User[];
  project: project.Project;
  selectedPeer: project.User;
}

const refreshExecutor = mutexExecutor.create();

const screenStore = remote.createStore<Screen>();
export const store = screenStore.readable;

export const fetch = (projectUrn: string): void => {
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

export const trackPeer = (projectUrn: string, peerId: PeerId): void => {
  proxy.client.project
    .peerTrack(projectUrn, peerId)
    .then(() => refreshPeers())
    .catch(err => screenStore.error(error.fromUnknown(err)));
};

export const untrackPeer = (projectUrn: string, peerId: PeerId): void => {
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
      validationMessage: "This remote is already being tracked",
    },
  ]
);

export const addPeer = async (
  projectId: string,
  newRemote: PeerId
): Promise<boolean> => {
  // This has to be awaited contrary to what tslint suggests, because we're
  // running async remote validations in in the background. If we remove the
  // async then the seed input form will have to be submitted twice to take any
  // effect.
  await peerValidation.validate(newRemote);
  if (get(peerValidation).type !== "valid") {
    return false;
  }

  trackPeer(projectId, newRemote);
  return true;
};

export const removePeer = (projectId: string, peerId: PeerId): void => {
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

function throwUnlessPeersPresent(
  peers: project.User[],
  projectId: string
): void {
  if (peers.length === 0) {
    throw new Error(`Project ${projectId} is missing peers`);
  }
}
