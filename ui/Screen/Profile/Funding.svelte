<script lang="ts">
  import * as svelteStore from "svelte/store";

  import {
    selectedEnvironment as ethereumEnvironment,
    supportedNetwork,
  } from "../../src/ethereum";
  import { store, Status } from "../../src/wallet";
  import * as pool from "../../src/funding/pool";

  import ConnectWallet from "../../DesignSystem/Component/Wallet/Connect.svelte";
  import WalletPanel from "../../DesignSystem/Component/Wallet/Panel.svelte";
  import WrongNetwork from "../../DesignSystem/Component/Wallet/WrongNetwork.svelte";

  import Pool from "../Funding/Pool.svelte";

  $: wallet = svelteStore.get(store);
</script>

<style>
  .container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: var(--content-padding);
    margin: 0 auto;

    display: flex;
    align-items: flex-start;
  }
</style>

{#if $wallet.status === Status.Connected}
  <div class="container">
    <WalletPanel
      onDisconnect={wallet.disconnect}
      account={$wallet.connected.account}
      style={'margin-right: var(--content-padding)'} />
    {#if supportedNetwork($ethereumEnvironment) === $wallet.connected.network}
      <Pool pool={pool.make(wallet)} />
    {:else}
      <WrongNetwork expectedNetwork={supportedNetwork($ethereumEnvironment)} />
    {/if}
  </div>
{:else}
  <ConnectWallet
    onConnect={wallet.connect}
    connecting={$wallet.status === Status.Connecting} />
{/if}
