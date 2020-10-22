<script lang="typescript">
  // import { push } from "svelte-spa-router";
  // import * as path from "../src/path";
  import * as transaction from "../src/transaction";

  import Center from "../DesignSystem/Component/Transaction/Center.svelte";

  const select = (_event: unknown) => {
    console.log("TODO(nuno): Implement this");
  }; // push(path.transactions(event.detail));

  let txs: transaction.Tx[] = [];
  $: transaction.store.subscribe(xs => {
    console.log("Received update to transactions");
    txs = xs;
  });
</script>

<style>
  .transaction-center {
    bottom: 32px;
    position: fixed;
    right: 32px;
    z-index: 900;
  }
</style>

{#if txs.length > 0}
  <div class="transaction-center">
    <Center on:select={select} transactions={txs} />
  </div>
{/if}
