<script>
  import { fly } from "svelte/transition";
  import { push } from "svelte-spa-router";

  import * as path from "../src/path.ts";
  import { summary, transactions as store } from "../src/transaction.ts";

  import Remote from "../DesignSystem/Component/Remote.svelte";

  import Center from "../DesignSystem/Component/Transaction/Center.svelte";

  const select = (event) => push(path.transactions(event.detail));

  $: centerIn = { delay: 240, duration: 320, x: 500 };
</script>

<style>
  .center {
    bottom: 32px;
    position: fixed;
    right: 32px;
    z-index: 900;
  }
</style>

<Remote {store} let:data={transactions}>
  {#if transactions.length > 0}
    <div class="center" in:fly={centerIn}>
      <Center on:select={select} summary={$summary} {transactions} />
    </div>
  {/if}
</Remote>
