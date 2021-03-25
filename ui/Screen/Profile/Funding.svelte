<script lang="ts">
  import { getContext } from "svelte";

  import * as attestation from "../../src/attestation";
  import {
    selectedEnvironment as ethereumEnvironment,
    supportedNetwork,
  } from "../../src/ethereum";
  import { ethereumAddress } from "../../src/identity";
  import { store, Status } from "../../src/wallet";
  import * as pool from "../../src/funding/pool";
  import type { UnsealedSession } from "../../src/session";

  import ConnectWallet from "../../DesignSystem/Component/Wallet/Connect.svelte";
  import WalletPanel from "../../DesignSystem/Component/Wallet/Panel.svelte";
  import WrongNetwork from "../../DesignSystem/Component/Wallet/WrongNetwork.svelte";

  import LinkAddress from "../Funding/LinkAddress.svelte";
  import Pool from "../Funding/Pool.svelte";
  import { claimsAddress, ClaimsContract } from "../../src/funding/contract";

  $: wallet = $store;
  // Hack to have Svelte working with checking the $wallet variant
  // and thus be able to access its appropriate fields.
  $: w = $wallet;

  const session = getContext("session") as UnsealedSession;
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
      {#await attestation.status(w.connected.account.address, session.identity, new ClaimsContract($wallet.signer, claimsAddress(wallet.environment)))}
        <p>...waiting</p>
      {:then attestationStatus}
        {#if attestationStatus === attestation.Status.Incomplete}
          <LinkAddress />
        {:else if attestationStatus === attestation.Status.Refuted}
          <!-- TODO(nuno): design this state with Brandon -->
          You connected with an address different than the one you have
          registered with. Re-connect using your registered ethereum address
          {$ethereumAddress}
        {:else}
          <Pool pool={pool.make(wallet)} />
        {/if}
      {/await}
    {:else}
      <WrongNetwork expectedNetwork={supportedNetwork($ethereumEnvironment)} />
    {/if}
  </div>
{:else}
  <ConnectWallet
    onConnect={wallet.connect}
    connecting={$wallet.status === Status.Connecting} />
{/if}
