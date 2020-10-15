<script lang="ts">
  import { build, Status } from "../../src/wallet";
  import { make } from "../../src/funding/pool";

  import { Button } from "../../DesignSystem/Primitive";
  import Pool from "../Funding/Pool.svelte";

  const wallet = build();
  // instantiate pool using the wallet store?
</script>

<div>
  {#if $wallet.status === Status.NotConnected}
    <Button on:click={wallet.connect}>connect</Button>
    {#if $wallet.error}Failed to connect: {$wallet.error.message}{/if}
  {:else if $wallet.status === Status.Connecting}
    <p>Connecting</p>
  {:else}
    <Button on:click={wallet.disconnect}>disconnect</Button>
    <div>Address: {$wallet.connected.account.address}</div>
    <div>Balance: {$wallet.connected.account.balance} eth</div>

    <Pool pool={make(wallet)} />
  {/if}
</div>
