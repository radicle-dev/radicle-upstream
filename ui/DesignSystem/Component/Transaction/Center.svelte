<script>
  import { slide } from "svelte/transition";
  import { IconState, summaryIconState } from "../../../src/transaction.ts";

  import List from "./List.svelte";
  import Summary from "./Summary.svelte";

  export let summary = null;
  export let transactions = null;

  // Transaction center element. Set by the view.
  let txList = null;
  let hidden = true;

  const toggleList = () => {
    hidden = !hidden;
  };

  const handleClick = ev => {
    // Any click *outside* the elem should hide the elem.
    if (txList !== ev.target && !txList.contains(ev.target) && !hidden) {
      hidden = true;
    }
  };

  $: negative = summaryIconState(summary.counts) === IconState.Negative;
</script>

<style>
  .center {
    bottom: 0;
    right: 0;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    box-shadow: var(--elevation-low);
    cursor: pointer;
    min-width: 275px;
    position: absolute;
    user-select: none;
    z-index: 900;
  }

  .negative {
    border: 1px solid var(--color-negative);
  }

  .list-wrapper {
    max-height: 80vh;
    overflow-y: auto;
  }
  .list-wrapper::-webkit-scrollbar {
    display: none; /* Chrome Safari */
  }

  .hidden {
    display: none;
  }
</style>

<svelte:window on:click={handleClick} />
<div
  bind:this={txList}
  class="center"
  class:negative
  data-cy="transaction-center">
  {#if !hidden}
    <div
      class="list-wrapper"
      class:hidden
      in:slide={{ duration: 360 }}
      out:slide={{ duration: 240 }}>
      <List on:select {transactions} />
    </div>
  {/if}
  <Summary on:click={toggleList} {summary} />
</div>
