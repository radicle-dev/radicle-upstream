<script>
  export let transactions = null;
  import { push } from "svelte-spa-router";
  import * as path from "../../../src/path.ts";

  import TransactionListItem from "./TxListItem.svelte";

  const select = event => push(path.transactions(event));
</script>

{#if transactions.length > 0}
  {#each transactions as tx}
    <TransactionListItem on:click={() => select(tx.id)} {tx} />
  {/each}
{/if}
