<script>
  import { transactions as store } from "../../../src/transaction.ts";
  import Remote from "../../../DesignSystem/Component/Remote.svelte";
</script>

<style>

</style>

<Remote {store} let:data={transactions}>
  {#if transactions.length > 0}
    {console.log(transactions)}
    {#each transactions as tx}
      <p>{tx.id}</p>
    {/each}
  {/if}
</Remote>
