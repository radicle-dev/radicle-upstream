// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { SvelteComponent } from "svelte";
import { Status, StatusType } from "ui/src/localPeer";
import { unreachable } from "ui/src/unreachable";

import { Icon } from "ui/DesignSystem";
import SyncingIcon from "ui/App/Sidebar/ConnectionStatusIndicator/Syncing.svelte";
import OfflineIcon from "ui/App/Sidebar/ConnectionStatusIndicator/Offline.svelte";

const connectedPeerCount = (peers: { [peerId: string]: string[] }): string => {
  const count = Object.keys(peers).length;
  return peerCount(count);
};

const peerCount = (count: number): string => {
  if (count === 1) {
    return "1 peer";
  } else {
    return `${count} peers`;
  }
};

interface IndicatorState {
  text: string;
  cy: string;
  icon: typeof SvelteComponent;
  fill: string;
}

export const indicatorState = (status: Status): IndicatorState => {
  if (status.type === StatusType.Online) {
    return {
      text: `You’re connected to ${connectedPeerCount(status.connectedPeers)}`,
      cy: "connection-status-online",
      icon: Icon.Network,
      fill: "var(--color-positive)",
    };
  } else if (status.type === StatusType.Syncing) {
    return {
      text: `Syncing with ${peerCount(
        status.syncs
      )} to get new content from your network`,
      cy: "connection-status-syncing",
      icon: SyncingIcon,
      fill: "var(--color-caution)",
    };
  } else if (
    status.type === StatusType.Offline ||
    status.type === StatusType.Started
  ) {
    return {
      text: "You’re not connected to any peers",
      cy: "connection-status-offline",
      icon: OfflineIcon,
      fill: "var(--color-negative)",
    };
  } else if (status.type === StatusType.Stopped) {
    return {
      text: "The app couldn't start your peer",
      cy: "connection-status-stopped",
      icon: OfflineIcon,
      fill: "var(--color-negative)",
    };
  } else {
    return unreachable(status);
  }
};
