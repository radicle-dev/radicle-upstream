<script>
  import { push } from "svelte-spa-router";
  import { transactions as store } from "../../../src/transaction.ts";
  import * as path from "../../../src/path.ts";

  import Remote from "../../../DesignSystem/Component/Remote.svelte";
  import TransactionListItem from "./TransactionListItem.svelte";

  const select = event => push(path.transactions(event));
</script>

<Remote {store} let:data={transactions}>
  {#if transactions.length > 0}
    {#each transactions as tx}
      <TransactionListItem on:click={() => select(tx.id)} {tx} />
    {/each}
  {/if}
</Remote>
