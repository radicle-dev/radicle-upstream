<script lang="ts">
  import {
    selectedEnvironment as ethereumEnvironment,
    supportedNetwork,
  } from "../../src/ethereum";
  import {
    watchAttestationStatus,
    attestationStatus,
    AttestationStatus,
  } from "../../src/attestation/status";
  import { store, Status } from "../../src/wallet";
  import * as pool from "../../src/funding/pool";

  import ConnectWallet from "../../DesignSystem/Component/Wallet/Connect.svelte";
  import WalletPanel from "../../DesignSystem/Component/Wallet/Panel.svelte";
  import WrongNetwork from "../../DesignSystem/Component/Wallet/WrongNetwork.svelte";
  import Pool from "../Funding/Pool.svelte";
  import LinkAddress from "../Funding/LinkAddress.svelte";

  watchAttestationStatus(store);

  $: wallet = $store;
  // Hack to have Svelte working with checking the $wallet variant
  // and thus be able to access its appropriate fields.
  $: w = $wallet;

</script>

<style>
  .container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: var(--content-padding);
    padding-bottom: 9.375rem;
    margin: 0 auto;

    display: flex;
    align-items: flex-start;
  }

</style>

{#if w.status === Status.Connected}
  <div class="container">
    <WalletPanel
      onDisconnect={wallet.disconnect}
      account={w.connected.account}
      style={'margin-right: var(--content-padding)'} />
    {#if supportedNetwork($ethereumEnvironment) === w.connected.network}
      {#if $attestationStatus === AttestationStatus.Fetching}
        Checking whether you have attested your Ethereum adddress...
      {:else if $attestationStatus === AttestationStatus.Valid}
        <Pool pool={pool.make(wallet)} />
      {:else}
        <LinkAddress />
      {/if}
    {:else}
      <WrongNetwork expectedNetwork={supportedNetwork($ethereumEnvironment)} />
    {/if}
  </div>
{:else}
  <ConnectWallet
    onConnect={wallet.connect}
    connecting={$wallet.status === Status.Connecting} />
{/if}
