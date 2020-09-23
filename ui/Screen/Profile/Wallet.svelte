<script>
  import { makeWallet } from "../../src/wallet";

  import { Button } from "../../DesignSystem/Primitive";

  const wallet = makeWallet();
</script>

<div>
  {#if $wallet.status === 'NOT_CONNECTED'}
    <Button on:click={wallet.connect}>connect</Button>
  {:else}
    <Button on:click={wallet.disconnect}>disconnect</Button>
    <br />
    <Button on:click={wallet.testSign}>test sign</Button>
    <br />
    <div>Address: {$wallet.connected.account.address}</div>
    <div>Balance: {$wallet.connected.account.balance} eth</div>
    <br />
  {/if}
</div>
