<script lang="typescript">
  import { status, StatusType } from "ui/src/localPeer";

  import { Icon } from "ui/DesignSystem/Primitive";
  import Tooltip from "ui/DesignSystem/Component/Tooltip.svelte";

  import Syncing from "./ConnectionStatusIndicator/Syncing.svelte";
  import Offline from "./ConnectionStatusIndicator/Offline.svelte";

  const connectedPeerCount = (peers: {
    [peerId: string]: string[];
  }): string => {
    const count = Object.keys(peers).length;
    return peerCount(count);
  };

  const peerCount = (count: number) => {
    if (count === 1) {
      return "1 peer";
    } else {
      return `${count} peers`;
    }
  };
</script>

<style>
  .item {
    width: var(--sidebar-width);
    height: 32px;
    margin-bottom: 16px;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: help;
  }
</style>

<div>
  {#if $status.type === StatusType.Online}
    <Tooltip
      value={`Network • You’re connected to ${connectedPeerCount(
        $status.connectedPeers
      )}`}>
      <div class="item indicator" data-cy="connection-status-online">
        <Icon.Network />
      </div>
    </Tooltip>
  {:else if $status.type === StatusType.Syncing}
    <Tooltip
      value={`Network • Syncing with ${peerCount(
        $status.syncs
      )} to get new content from your network`}>
      <div class="item indicator" data-cy="connection-status-syncing">
        <Syncing />
      </div>
    </Tooltip>
  {:else if $status.type === StatusType.Offline || $status.type === StatusType.Started}
    <Tooltip value="Network • You’re not connected to any peers">
      <div class="item indicator" data-cy="connection-status-offline">
        <Offline />
      </div>
    </Tooltip>
  {:else if $status.type === StatusType.Stopped}
    <Tooltip value="Network • The app couldn't start your peer">
      <div class="item indicator" data-cy="connection-status-stopped">
        <Offline style="fill: var(--color-negative);" />
      </div>
    </Tooltip>
  {/if}
</div>
