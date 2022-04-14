// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { get } from "svelte/store";

import * as error from "ui/src/error";
import * as mutexExecutor from "ui/src/mutexExecutor";
import * as project from "ui/src/project";
import * as proxy from "ui/src/proxy";
import * as remote from "ui/src/remote";

interface Screen {
  peers: project.Peer[];
  peerSelection: project.User[];
  project: project.Project;
  selectedPeer: project.User;
}

export const VALID_PEER_MATCH = /[1-9A-HJ-NP-Za-km-z]{54}/;
export const screenRemoteStore = remote.createStore<Screen>();
export const store = screenRemoteStore.readable;

export function fetch(projectUrn: string): void {
  screenRemoteStore.loading();

  proxy.client.project
    .get(projectUrn)
    .then(async prj => {
      const peers = await proxy.client.project.listPeers(projectUrn);
      const peerSelection = project.userList(peers);
      throwUnlessPeersPresent(peerSelection, projectUrn);
      screenRemoteStore.success({
        peers,
        peerSelection,
        project: prj,
        selectedPeer: peerSelection[0],
      });
    })
    .catch(err => screenRemoteStore.error(error.fromUnknown(err)));
}

export function removePeer(projectId: string, peerId: string): void {
  const screen = get(screenRemoteStore);

  if (screen.status === remote.Status.Success) {
    const { peerSelection, selectedPeer } = screen.data;

    proxy.client.project
      .peerUntrack(projectId, peerId)
      .then(() => refreshPeers())
      .catch(err => screenRemoteStore.error(error.fromUnknown(err)));

    if (selectedPeer.peerId === peerId) {
      screenRemoteStore.success({
        ...screen.data,
        selectedPeer: peerSelection[0],
      });
    }
  }
}

export function addPeer(projectId: string, newRemote: string): void {
  proxy.client.project
    .peerTrack(projectId, newRemote)
    .then(() => refreshPeers())
    .catch(err => screenRemoteStore.error(error.fromUnknown(err)));
}

const refreshExecutor = mutexExecutor.create();

export async function refreshPeers(): Promise<void> {
  const screen = get(screenRemoteStore);

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
      screenRemoteStore.success({
        ...screen.data,
        peers,
        peerSelection,
      });
    } catch (err: unknown) {
      screenRemoteStore.error(error.fromUnknown(err));
    }
  }
}

export function selectPeer(peer: project.User): void {
  const screen = get(screenRemoteStore);

  if (screen.status === remote.Status.Success) {
    const { data: current } = screen;

    if (peer.peerId !== current.selectedPeer.peerId) {
      screenRemoteStore.success({ ...current, selectedPeer: peer });
    }
  }
}

function throwUnlessPeersPresent(
  peers: project.User[],
  projectId: string
): void {
  if (peers.length === 0) {
    throw new Error(`Project ${projectId} is missing peers`);
  }
}
