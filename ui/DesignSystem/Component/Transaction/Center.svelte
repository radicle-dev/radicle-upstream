<script>
  import { IconState, summaryIconState } from "../../../src/transaction.ts";

  import List from "./List.svelte";
  import Summary from "./Summary.svelte";

  export let summary = null;
  export let transactions = null;

  let hidden = true;

  const toggleList = () => {
    hidden = !hidden;
  };

  const negative = summaryIconState(summary.counts) === IconState.Negative;
</script>

<style>
  .center {
    bottom: 32px;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    box-shadow: var(--elevation-low);
    cursor: pointer;
    min-width: 275px;
    position: absolute;
    right: 32px;
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

<div class="center" class:negative data-cy="transaction-center">
  <div class="list-wrapper" class:hidden>
    <List on:select {transactions} />
  </div>
  <Summary {summary} on:click={toggleList} />
</div>
