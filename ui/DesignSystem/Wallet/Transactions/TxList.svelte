<script lang="typescript">
  import * as modal from "ui/src/modal";
  import { selectedStore } from "ui/src/transaction";
  import type { Tx } from "ui/src/transaction";

  import ModalTransaction from "ui/Modal/Transaction.svelte";
  import TxListItem from "./TxListItem.svelte";

  export let title: string;
  export let txs: Tx[];

  const onSelect = (hash: string) => {
    selectedStore.set(hash);
    modal.hide();
    modal.toggle(ModalTransaction);
  };
</script>

<style>
  .title-row {
    display: flex;
    flex: 1;
    padding: 0.25rem 0.75rem;
    color: var(--color-foreground-level-5);
    background-color: var(--color-foreground-level-1);
    border-bottom: 1px solid var(--color-foreground-level-2);
  }

  .title-row:first-child {
    border-top-left-radius: 0.4rem;
    border-top-right-radius: 0.4rem;
  }
</style>

<h5 class="title-row">{title}</h5>
{#each txs as tx}
  <TxListItem on:click={() => onSelect(tx.hash)} {tx} />
{/each}
