<script lang="typescript">
  import type { Tx } from "../../../src/transaction";
  import { TxStatus, summaryCounts } from "../../../src/transaction";

  import List from "./Center/List.svelte";
  import Summary from "./Center/Summary.svelte";

  export let transactions: Tx[] = [];

  // Transaction center element. Set by the view. Should be of type 'HTMLDivElement'.
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let element: any;
  let expand = false;
  let initialized = false;

  // The set of transaction hashes that have already been displayed
  // in the expanded transaction stack.
  let displayedTxs: Set<string> = new Set();

  if (!initialized) {
    displayedTxs = new Set(transactions.map(tx => tx.hash));
    initialized = false;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const toggleStack = (ev: any) => {
    expand = expand
      ? false
      : element === ev.target || element.contains(ev.target);
    transactions.forEach(tx => displayedTxs.add(tx.hash));
  };

  $: expand = expand || transactions.some(tx => !displayedTxs.has(tx.hash));
  $: negative = transactions.some(tx => tx.status === TxStatus.Rejected);
</script>

<style>
  .transaction-center {
    background: var(--color-background);
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    box-shadow: var(--elevation-medium);
    cursor: pointer;
    height: 3.5rem;
    min-width: 17.2rem;
    overflow: hidden;
    transition: height 360ms ease;
    user-select: none;
  }

  .transaction-center:hover {
    background: var(--color-foreground-level-1);
  }

  .transaction-center.expand {
    /* The list height minus the height of the height of the .transaction-center */
    height: calc(var(--list-height) - 3.5rem);
  }

  .negative {
    border: 1px solid var(--color-negative);
  }

  .stack {
    max-height: 80vh;
    overflow-y: auto;
    transform: translateY(calc(-1 * calc(100% - 3.5rem)));
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
  class:negative>
  <div class="stack" class:expand>
    <List on:select {transactions} />
    <Summary counts={summaryCounts(transactions)} />
  </div>
</div>
