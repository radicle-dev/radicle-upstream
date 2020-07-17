<script>
  export let transactions = null;
  import { push } from "svelte-spa-router";
  import * as path from "../../../src/path.ts";

  import TransactionListItem from "./TxListItem.svelte";
  import { Text } from "../../Primitive";

  const select = transactionId => push(path.transactions(transactionId));
</script>

<style>
  .empty-state {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 5rem;
    border-top: 1px solid var(--color-foreground-level-2);
  }
</style>

{#if transactions.length > 0}
  {#each transactions as tx}
    <TransactionListItem on:click={() => select(tx.id)} {tx} />
  {/each}
{:else}
  <div class="empty-state">
    <Text>No transactions yet</Text>
  </div>
{/if}
