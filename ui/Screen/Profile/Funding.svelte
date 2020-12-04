<script lang="ts">
  import { wallet, Status } from "../../src/wallet";
  import * as pool from "../../src/funding/pool";

  import ConnectWallet from "../../DesignSystem/Component/Wallet/Connect.svelte";
  import WalletPanel from "../../DesignSystem/Component/Wallet/Panel.svelte";

  import Pool from "../Funding/Pool.svelte";
</script>

<style>
  .funding-container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: var(--content-padding);
    margin: 0 auto;
  }

  .container {
    display: flex;
    align-items: flex-start;
  }
</style>

<div class="funding-container">
  {#if $wallet.status === Status.Connected}
    <div class="container">
      <WalletPanel
        onDisconnect={wallet.disconnect}
        account={$wallet.connected.account}
        style={'margin-right: var(--content-padding)'} />
      <Pool pool={pool.make(wallet)} />
    </div>
  {:else}
    <ConnectWallet
      onConnect={wallet.connect}
      connecting={$wallet.status === Status.Connecting} />
  {/if}
</div>
