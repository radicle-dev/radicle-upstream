<script lang="typescript">
  import * as modal from "../src/modal";
  import { store as transactions, selectedStore } from "../src/transaction";

  import Center from "../DesignSystem/Component/Transaction/Center.svelte";
  import ModalTransaction from "../Modal/Transaction.svelte";

  const onSelect = (event: { detail: string }) => {
    const hash = event.detail;
    selectedStore.set(hash);
    modal.hide();
    modal.toggle(ModalTransaction);
  };

</script>

<style>
  .transaction-center {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    z-index: 50;
  }

</style>

{#if $transactions.length > 0}
  <div class="transaction-center">
    <Center on:select={onSelect} transactions={$transactions} />
  </div>
{/if}
