<script>
  import { status as store, StatusType } from "../../src/localPeer.ts";

  import Remote from "../Component/Remote.svelte";
  import { Icon } from "../Primitive";
  import Tooltip from "./Tooltip.svelte";

  import Syncing from "./ConnectionStatusIndicator/Syncing.svelte";
  import Offline from "./ConnectionStatusIndicator/Offline.svelte";

  const peerCount = count => {
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

<Remote {store} let:data={status}>
  <div>
    {#if status.type === StatusType.Online}
      <Tooltip value={`You’re connected to ${peerCount(status.connected)}`}>
        <div class="item indicator" data-cy="connection-status-online">
          <Icon.Network />
        </div>
      </Tooltip>
    {:else if status.type === StatusType.Syncing}
      <Tooltip
        value={`Syncing with ${peerCount(status.syncs)} to get new content from your network`}>
        <div class="item indicator" data-cy="connection-status-syncing">
          <Syncing />
        </div>
      </Tooltip>
    {:else if status.type === StatusType.Offline || status.type === StatusType.Started}
      <Tooltip value="You’re not connected to any peers">
        <div class="item indicator" data-cy="connection-status-offline">
          <Offline />
        </div>
      </Tooltip>
    {:else if status.type === StatusType.Stopped}
      <Tooltip value="The app couldn't start your peer">
        <div class="item indicator" data-cy="connection-status-stopped">
          <Offline style="fill: var(--color-negative);" />
        </div>
      </Tooltip>
    {/if}
  </div>
</Remote>
