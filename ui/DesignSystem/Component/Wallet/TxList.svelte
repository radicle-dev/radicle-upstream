<script>
  import { push } from "svelte-spa-router";
  import * as path from "../../../src/path.ts";

  import TransactionListItem from "./TxListItem.svelte";
  import EmptyState from "./../EmptyState.svelte";

  export let transactions = null;
  export let accountId = null;

  const select = transactionId =>
    push(path.transactions(transactionId, accountId));
</script>

{#if transactions.length > 0}
  {#each transactions as tx}
    <TransactionListItem
      on:click={() => select(tx.id, accountId)}
      {tx}
      {accountId} />
  {/each}
{:else}
  <EmptyState
    text="No transactions yet"
    style="height: 100%; padding: 2rem 0 1rem;" />
{/if}
