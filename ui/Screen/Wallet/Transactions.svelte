<script lang="typescript">
  import * as modal from "ui/src/modal";
  import { store as transactions, selectedStore } from "ui/src/transaction";

  import TxItem from "ui/DesignSystem/Component/Wallet/Transactions/TxItem.svelte";
  import ModalTransaction from "ui/Modal/Transaction.svelte";

  const onSelect = (hash: string) => {
    selectedStore.set(hash);
    modal.hide();
    modal.toggle(ModalTransaction);
  };

  $: pendingTxs = $transactions.filter(tx => tx.status !== "Included");
</script>

<style>
  .list {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
  }

  .title-row {
    display: flex;
    flex: 1;
    padding: 0.25rem 0.75rem;
    color: var(--color-foreground-level-5);
    background-color: var(--color-foreground-level-1);
    border-bottom: 1px solid var(--color-foreground-level-2);
  }
</style>

<div>
  {#if $transactions.length > 0}
    {#if pendingTxs.length > 0}
      <div class="list">
        <h5 class="title-row">Pending transactions</h5>
        {#each pendingTxs as tx}
          <TxItem on:click={() => onSelect(tx.hash)} {tx} />
        {/each}
      </div>
    {/if}
    <div class="list">
      <h5 class="title-row">2021</h5>
      {#each $transactions as tx}
        <TxItem on:click={() => onSelect(tx.hash)} {tx} />
      {/each}
    </div>
  {/if}
</div>
