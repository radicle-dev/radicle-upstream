<script>
  import { slide } from "svelte/transition";

  import { IconState, summaryIconState } from "../../../src/transaction.ts";

  import List from "./List.svelte";
  import Summary from "./Summary.svelte";

  export let summary = null;
  export let transactions = null;

  let show = false;

  const toggleList = () => {
    show = !show;
  };

  $: listIn = { delay: 180, duration: 350 };
  $: listOut = { duration: 280 };
  $: negative = summaryIconState(summary.counts) === IconState.Negative;
</script>

<style>
  .center {
    border: 1px solid var(--color-foreground-level-3);
    bottom: 0;
    border-radius: 4px;
    box-shadow: var(--elevation-low);
    cursor: pointer;
    min-width: 275px;
    position: absolute;
    right: 0;
    user-select: none;
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
</style>

<div class="center" class:negative data-cy="transaction-center">
  {#if show}
    <div class="list-wrapper" in:slide={listIn} out:slide={listOut}>
      <List on:select {transactions} />
    </div>
  {/if}
  <Summary on:click={toggleList} {summary} />
</div>
