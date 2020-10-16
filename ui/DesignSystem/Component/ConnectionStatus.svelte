<script>
  import { status as store, StatusType } from "../../src/localPeer.ts";

  import Remote from "../Component/Remote.svelte";
  import { Icon } from "../Primitive";
  import Tooltip from "./Tooltip.svelte";

  import Syncing from "./ConnectionStatus/Syncing.svelte";
  import Offline from "./ConnectionStatus/Offline.svelte";
</script>

<style>
  a {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }
</style>

<Remote {store} let:data={status}>
  <div>
    {#if status.type === StatusType.Online}
      <Tooltip value={`You’re connected to ${status.connected} peers`}>
        <!-- svelte-ignore a11y-missing-attribute -->
        <a>
          <Icon.Network />
        </a>
      </Tooltip>
    {:else if status.type === StatusType.Syncing}
      <Tooltip
        value={`Syncing with ${status.syncs} peers to get new content from your network`}>
        <!-- svelte-ignore a11y-missing-attribute -->
        <a>
          <Syncing />
        </a>
      </Tooltip>
    {:else if status.type === StatusType.Offline || status.type === StatusType.Started}
      <Tooltip value="You’re not connected to any peers">
        <!-- svelte-ignore a11y-missing-attribute -->
        <a>
          <Offline />
        </a>
      </Tooltip>
    {:else if status.type === StatusType.Stopped}
      <Tooltip value="The app couldn't start your peer">
        <!-- svelte-ignore a11y-missing-attribute -->
        <a>
          <Offline style="fill: var(--color-negative);" />
        </a>
      </Tooltip>
    {/if}
  </div>
</Remote>
