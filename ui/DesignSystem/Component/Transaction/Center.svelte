<script lang="typescript">
  import type { SvelteComponent } from "svelte";

  import type { Tx } from "../../../src/transaction";
  import { TxStatus, summaryCounts } from "../../../src/transaction";

  import List from "./List.svelte";
  import Summary from "./Summary.svelte";

  //   export let summary = null;
  export let transactions: Tx[] = [];

  // Transaction center element. Set by the view.
  let element: SvelteComponent;
  let expand = true;
  const stack = null;

  const toggleStack = ev => {
    expand = expand
      ? false
      : element === ev.target || element.contains(ev.target);
  };

  $: negative = !!transactions.find(tx => tx.status === TxStatus.Rejected);
</script>

<style>
  .transaction-center {
    background: var(--color-background);
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 8px;
    box-shadow: var(--elevation-medium);
    cursor: pointer;
    height: 56px;
    min-width: 275px;
    overflow: hidden;
    transition: height 360ms ease;
    user-select: none;
  }

  .transaction-center:hover {
    background: var(--color-foreground-level-1);
  }

  .transaction-center.expand {
    height: calc(var(--list-height) - 56px);
  }

  .negative {
    border: 1px solid var(--color-negative);
  }

  .stack {
    max-height: 80vh;
    overflow-y: auto;
    transform: translateY(calc(-1 * calc(100% - 56px)));
    transition: transform 360ms ease;
  }
  .stack::-webkit-scrollbar {
    display: none; /* Chrome Safari */
  }

  .stack.expand {
    transform: translateY(0px);
  }
</style>

<svelte:window on:click={toggleStack} />
<div
  bind:this={element}
  class="transaction-center"
  data-cy="transaction-center"
  class:expand
  class:negative
  style="--list-height: {stack && stack.offsetHeight}px">
  <div class="stack" class:expand>
    <List {transactions} />
    <Summary counts={summaryCounts(transactions)} />
  </div>
</div>
