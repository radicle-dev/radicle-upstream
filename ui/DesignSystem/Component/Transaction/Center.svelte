<script>
  import { IconState, summaryIconState } from "../../../src/transaction.ts";

  import List from "./List.svelte";
  import Summary from "./Summary.svelte";

  export let summary = null;
  export let transactions = null;

  // Transaction center element. Set by the view.
  let element = null;
  let expand = false;
  let list = null;

  const toggleList = ev => {
    if ((element === ev.target || element.contains(ev.target)) && !expand) {
      expand = true;
    } else if (expand) {
      expand = false;
    }
  };

  $: negative = summaryIconState(summary.counts) === IconState.Negative;
  $: console.log(list && list.offsetHeight);
</script>

<style>
  .center {
    background: var(--color-background);
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 8px;
    box-shadow: var(--elevation-medium);
    cursor: pointer;
    height: 56px;
    min-width: 275px;
    overflow: hidden;
    transition: height 360ms ease;
    user-select: none;
  }

  .center:hover {
    background: var(--color-foreground-level-1);
  }

  .center.expand {
    height: calc(var(--list-height) - 56px);
  }

  .negative {
    border: 1px solid var(--color-negative);
  }

  .list {
    max-height: 80vh;
    overflow-y: auto;
    transform: translateY(calc(-1 * calc(100% - 56px)));
    transition: transform 360ms ease;
  }
  .list::-webkit-scrollbar {
    display: none; /* Chrome Safari */
  }

  .list.expand {
    transform: translateY(0px);
  }
</style>

<svelte:window on:click={toggleList} />
<div
  bind:this={element}
  class="center"
  class:expand
  class:negative
  data-cy="transaction-center"
  style="--list-height: {list && list.offsetHeight}px">
  <div bind:this={list} class="list" class:expand>
    <List on:select {transactions} />
    <Summary {summary} />
  </div>
</div>
