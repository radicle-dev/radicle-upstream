// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Identity } from "proxy-client/identity";

import { get } from "svelte/store";
import lodash from "lodash";

import * as Project from "ui/src/project";
import * as error from "ui/src/error";
import * as localPeer from "ui/src/localPeer";
import * as mutexExecutor from "ui/src/mutexExecutor";
import * as proxy from "ui/src/proxy";
import * as remote from "ui/src/remote";
import * as session from "ui/src/session";

interface Screen {
  peers: Project.Peer[];
  peerSelection: Project.User[];
  project: Project.Project;
  selectedPeer: Project.User;
}

export const VALID_PEER_MATCH = /^[1-9A-HJ-NP-Za-km-z]{54}$/;
export const screenRemoteStore = remote.createStore<Screen>();
export const store = screenRemoteStore.readable;

let currentUrn: string = "";

const fetchExecutor = mutexExecutor.create();
export async function fetch(projectUrn: string): Promise<void> {
  if (currentUrn !== projectUrn) {
    screenRemoteStore.loading();
  }
  currentUrn = projectUrn;

  try {
    const response = await fetchExecutor.run(async abort => {
      const project = await proxy.client.project.get(projectUrn, { abort });
      const peers = await proxy.client.project.listPeers(projectUrn, { abort });
      return { project, peers };
    });

    if (response) {
      const peerSelection = Project.userList(response.peers);
      throwUnlessPeersPresent(peerSelection, projectUrn);

      screenRemoteStore.success({
        peers: response.peers,
        peerSelection,
        project: response.project,
        selectedPeer: peerSelection[0],
      });
    }
  } catch (err: unknown) {
    screenRemoteStore.error(error.fromUnknown(err));
  }
}

const fetchThrottled: (projectId: string) => void = lodash.throttle(
  fetch,
  1000 // 1 second
);

export function watchProjectUpdates(projectId: string): () => void {
  return localPeer.projectEvents.onValue(event => {
    const screen = get(store);
    if (screen.status === remote.Status.Success) {
      if (event.urn.startsWith(projectId)) {
        fetchThrottled(projectId);
      }
    }
  });
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

      const peerSelection = Project.userList(peers);
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

export function selectPeer(peer: Project.User): void {
  const screen = get(screenRemoteStore);

  if (screen.status === remote.Status.Success) {
    const { data: current } = screen;

    if (peer.peerId !== current.selectedPeer.peerId) {
      screenRemoteStore.success({ ...current, selectedPeer: peer });
    }
  }
}

function throwUnlessPeersPresent(
  peers: Project.User[],
  projectId: string
): void {
  if (peers.length === 0) {
    throw new Error(`Project ${projectId} is missing peers`);
  }
}

export function getUserForPeerId(peerId: string): Identity | undefined {
  const result = get(store);
  if (result.status !== remote.Status.Success) {
    return undefined;
  }

  const user = result.data.peerSelection.find(p => p.peerId === peerId);
  if (user) {
    return user.identity;
  } else {
    const ownIdentity = session.unsealed().identity;
    if (ownIdentity.peerId === peerId) {
      return ownIdentity;
    }
  }
}
