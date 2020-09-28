<script lang="ts">
  import { build, Status } from "../../src/wallet";

  import { Button } from "../../DesignSystem/Primitive";
  import Funding from "../Funding.svelte";

  const wallet = build();
</script>

<div>
  {#if $wallet.status === Status.NotConnected}
    <Button on:click={wallet.connect}>connect</Button>
  {:else}
    <Button on:click={wallet.disconnect}>disconnect</Button>
    <br />
    <Button on:click={_ => wallet.testTransfer(42)}>test sign</Button>
    <br />
    <div>Address: {$wallet.connected.account.address}</div>
    <div>Balance: {$wallet.connected.account.balance} eth</div>
    <br />
  {/if}
</div>

<br />
<Funding {wallet} />
