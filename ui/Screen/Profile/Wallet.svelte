<script>
  import { getContext } from "svelte";
  import { writable } from "svelte/store";
  import { wallet } from "../../src/wallet"
  import { Button } from "../../DesignSystem/Primitive"
  console.log(wallet)
</script>

<div>
  {#if $wallet.status === "NOT_CONNECTED"}
    <Button on:click={wallet.connect}>connect</Button>
  {:else if $wallet.status === "CONNECTING"}
    <Button on:click={wallet.disconnect}>disconnect</Button>
    Connecting
  {:else if $wallet.status === "CONNECTED"}
    <Button on:click={wallet.disconnect}>disconnect</Button>
    <div>is connected</div>
    <div>Chain ID: {$wallet.connected.chainId}</div>
    <div>Address: {$wallet.connected.account.address}</div>
    <div>Balance: {$wallet.connected.account.balance}</div>
  {/if}
</div>
