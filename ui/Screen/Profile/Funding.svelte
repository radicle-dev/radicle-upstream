<script lang="ts">
  import { build, Status } from "../../src/wallet";
  import * as pool from "../../src/funding/pool";

  import { Button } from "../../DesignSystem/Primitive";

  import Pool from "../Funding/Pool.svelte";

  const wallet = build();

  let txs: pool.Tx[] = [];

  $: pool.transactions.subscribe(xs => {
    console.log("Updates to transactions");
    txs = xs;
  });
</script>

<style>
  .funding-container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
    margin: 0 auto;
  }
</style>

<div class="funding-container">
  {#if $wallet.status === Status.NotConnected}
    <Button on:click={wallet.connect}>connect</Button>
    {#if $wallet.error}Failed to connect: {$wallet.error.message}{/if}
  {:else if $wallet.status === Status.Connecting}
    <p>Connecting</p>
  {:else}
    <Button on:click={wallet.disconnect}>disconnect</Button>
    <div>Address: {$wallet.connected.account.address}</div>
    <div>Balance: {$wallet.connected.account.balance} eth</div>

    <Pool pool={pool.make(wallet)} />
  {/if}
</div>

<ul>
  {#each txs as tx}
    <li>Status: {tx.status} | Hash: {tx.hash} | {tx.inner.kind}</li>
  {/each}
</ul>
