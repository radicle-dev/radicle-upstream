<script lang="ts">
  import * as svelteStore from "svelte/store";

  import { build, Status } from "../../src/wallet";
  import * as pool from "../../src/funding/pool";

  import { Button } from "../../DesignSystem/Primitive";
  import ConnectWallet from "../../DesignSystem/Component/Wallet/Connect.svelte";
  import WalletPanel from "../../DesignSystem/Component/Wallet/Panel.svelte";

  import Pool from "../Funding/Pool.svelte";

  const wallet = build();
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
  {#if $wallet.status === Status.Connected}
    <WalletPanel
      onDisconnect={wallet.disconnect}
      account={$wallet.connected.account} />
    <Pool pool={pool.make(wallet)} />
  {:else}
    <ConnectWallet
      onConnect={wallet.connect}
      connecting={$wallet.status === Status.Connecting} />
  {/if}
</div>
