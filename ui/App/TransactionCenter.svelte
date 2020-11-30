<script lang="typescript">
  import * as modal from "../src/modal";
  import * as path from "../src/path";
  import { store as transactions, selectedStore } from "../src/transaction";

  import Center from "../DesignSystem/Component/Transaction/Center.svelte";

  const onSelect = (event: { detail: string }) => {
    const hash = event.detail;
    selectedStore.set(hash);
    modal.hide();
    modal.toggle(path.transaction());
  };
</script>

<style>
  .transaction-center {
    bottom: 32px;
    position: fixed;
    right: 32px;
    z-index: 900;
  }
</style>

{#if $transactions.length > 0}
  <div class="transaction-center">
    <Center on:select={onSelect} transactions={$transactions} />
  </div>
{/if}
