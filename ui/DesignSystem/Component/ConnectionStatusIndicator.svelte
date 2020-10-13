<script lang="ts">
  export let state = "online"; // stopped | offline | syncing | online

  import { Icon } from "../Primitive";
  import Syncing from "./ConnectionStatusIndicator/Syncing.svelte";
  import Offline from "./ConnectionStatusIndicator/Offline.svelte";
  import Tooltip from "./Tooltip.svelte";

  // offline: four dots grey
  // stopped: four dots red (tooltip)
  // syncing: blinking dotted line
  // online: network icon

  // stopped/offline -> syncing: transition-in class (1sec)
  // stopped/offline -> online: transition-in class (1sec)
  // syncing -> online: dotted opacity 0 lines opacity 1 (0sec)

  const states = ["stopped", "offline", "syncing", "online"];

  const nextState = () => {
    const state = states.shift();
    states.push(state);
    return state;
  };
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

<div
  on:click={() => {
    state = nextState();
  }}>
  {#if state === 'online'}
    <Tooltip value="You’re connected to 17 peers">
      <!-- svelte-ignore a11y-missing-attribute -->
      <a>
        <Icon.Network />
      </a>
    </Tooltip>
  {:else if state === 'syncing'}
    <Tooltip value="Syncing new content from your network">
      <!-- svelte-ignore a11y-missing-attribute -->
      <a>
        <Syncing />
      </a>
    </Tooltip>
  {:else if state === 'offline'}
    <Tooltip value="You’re not connected to any peers">
      <!-- svelte-ignore a11y-missing-attribute -->
      <a>
        <Offline />
      </a>
    </Tooltip>
  {:else if state === 'stopped'}
    <Tooltip value="The app couldn’t establish a connection to your peers">
      <!-- svelte-ignore a11y-missing-attribute -->
      <a>
        <Offline style="fill: var(--color-negative);" />
      </a>
    </Tooltip>
  {/if}
</div>
