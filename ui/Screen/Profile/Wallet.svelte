<script lang="ts">
  import { build, Status } from "../../src/wallet";

  import { Button } from "../../DesignSystem/Primitive";
  import Funding from "../Funding.svelte";

  const wallet = build();
  // instantiate pool using the wallet store?
</script>

<div>
  {#if $wallet.status === Status.NotConnected}
    <Button on:click={wallet.connect}>connect</Button>
  {:else if $wallet.status === Status.Connecting}
    <p>Connecting</p>
    <Button on:click={wallet.disconnect}>Disconnect</Button>
  {:else}
    <Button on:click={wallet.disconnect}>disconnect</Button>
    <div>Address: {$wallet.connected.account.address}</div>
    <div>Balance: {$wallet.connected.account.balance} eth</div>
    <Funding {wallet} />
  {/if}
</div>
