<script>
  export let peerState;

  $: {
    console.log(peerState);

    if (peerState) {
      state = peerState.type;
    }
  }

  let state = "offline"; // stopped | offline | syncing | online

  import { Icon } from "../Primitive";
  import Syncing from "./ConnectionStatusIndicator/Syncing.svelte";
  import Offline from "./ConnectionStatusIndicator/Offline.svelte";
  import Tooltip from "./Tooltip.svelte";
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

<div>
  {#if state === 'online'}
    <Tooltip
      value={`You’re connected to ${peerState && peerState.connected} peers`}>
      <!-- svelte-ignore a11y-missing-attribute -->
      <a>
        <Icon.Network />
      </a>
    </Tooltip>
  {:else if state === 'syncing'}
    <Tooltip
      value={`Syncing with ${peerState && peerState.syncs} peers to get new content from your network`}>
      <!-- svelte-ignore a11y-missing-attribute -->
      <a>
        <Syncing />
      </a>
    </Tooltip>
  {:else if state === 'offline' || state === 'started'}
    <Tooltip value="You’re not connected to any peers">
      <!-- svelte-ignore a11y-missing-attribute -->
      <a>
        <Offline />
      </a>
    </Tooltip>
  {:else if state === 'stopped'}
    <Tooltip value="The app couldn't start your peer">
      <!-- svelte-ignore a11y-missing-attribute -->
      <a>
        <Offline style="fill: var(--color-negative);" />
      </a>
    </Tooltip>
  {/if}
</div>
