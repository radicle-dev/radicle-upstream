<script>
  import { push } from "svelte-spa-router";

  import * as path from "../src/path.ts";
  import { summary, transactions as store } from "../src/transaction.ts";

  import Remote from "../DesignSystem/Component/Remote.svelte";

  import Center from "../DesignSystem/Component/Transaction/Center.svelte";

  const select = event => push(path.transactions(event.detail));
</script>

<Remote {store} let:data={transactions}>
  {#if transactions.length > 0}
    <Center on:select={select} summary={$summary} {transactions} />
  {/if}
</Remote>
