<script lang="typescript">
  import * as modal from "../src/modal";
  import * as path from "../src/path";
  import * as transaction from "../src/transaction";

  import Center from "../DesignSystem/Component/Transaction/Center.svelte";

  const onSelect = (event: { detail: string }) => {
    const hash = event.detail;
    transaction.selectedStore.set(hash);
    modal.hide();
    modal.toggle(path.transaction());
  };

  let txs: transaction.Tx[] = [];
  $: transaction.store.subscribe(xs => {
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
    <Center on:select={onSelect} transactions={txs} />
  </div>
{/if}
