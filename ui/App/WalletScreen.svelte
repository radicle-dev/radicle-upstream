<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as router from "ui/src/router";
  import { unreachable } from "ui/src/unreachable";
  import * as modal from "ui/src/modal";
  import * as error from "ui/src/error";

  import {
    selectedEnvironment as ethereumEnvironment,
    supportedNetwork,
  } from "ui/src/ethereum";
  import {
    watchAttestationStatus,
    attestationStatus,
    AttestationStatus,
  } from "ui/src/attestation/status";
  import { store, Status, accountBalancesStore } from "ui/src/wallet";

  import ConnectWallet from "./WalletScreen/ConnectWallet.svelte";
  import LinkAddress from "./WalletScreen/LinkAddress.svelte";
  import QrCodeModal from "./WalletScreen/QrCodeModal.svelte";
  import Transactions from "./WalletScreen/Transactions.svelte";
  import WalletPanel from "./WalletScreen/WalletPanel.svelte";
  import WrongNetwork from "./WalletScreen/WrongNetwork.svelte";

  import { Icon } from "ui/DesignSystem";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  export let activeTab: router.WalletTab;

  const tabs = (active: router.WalletTab) => {
    const items = [
      {
        title: "Transactions",
        icon: Icon.Transactions,
        active: active === "transactions",
        onClick: () => {
          router.push({ type: "wallet", activeTab: "transactions" });
        },
      },
    ];

    return items;
  };

  watchAttestationStatus(store);

  $: wallet = $store;
  // Hack to have Svelte working with checking the $wallet variant
  // and thus be able to access its appropriate fields.
  $: w = $wallet;

  async function connectWallet() {
    try {
      await wallet.connect({
        show(uri: string, onClose: () => void) {
          modal.toggle(
            QrCodeModal,
            () => {
              onClose();
            },
            {
              uri,
            }
          );
        },
      });
      modal.hide();
    } catch (err: unknown) {
      error.show(
        new error.Error({
          message: "Failed to connect to wallet",
          source: err,
        })
      );
    }
  }
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    margin: 0 auto;
    padding: 2rem var(--content-padding);
  }
  .content {
    display: grid;
    grid-template-columns: 20rem auto;
    gap: 1.5rem;
  }
  .title {
    padding-left: 0.75rem;
    margin-bottom: 1rem;
  }
</style>

<ScreenLayout>
  <div class="container" data-cy="wallet">
    {#if w.status === Status.Connected}
      <div class="content">
        <div>
          <h1 class="title">Wallet</h1>
          <WalletPanel
            onDisconnect={wallet.disconnect}
            eth={$accountBalancesStore.eth}
            rad={$accountBalancesStore.rad}
            address={w.connected.address} />
        </div>
        {#if supportedNetwork($ethereumEnvironment) === w.connected.network}
          {#if $attestationStatus === AttestationStatus.Fetching}
            <EmptyState
              text="Checking whether you have attested your Ethereum address…"
              style="height: 30rem; margin-top: 3.75rem;"
              emoji="🧦" />
          {:else if $attestationStatus === AttestationStatus.Valid}
            <div class="right-column">
              <TabBar
                slot="left"
                tabs={tabs(activeTab)}
                style="padding: 0.5rem 0; margin-bottom: 1rem;" />
              {#if activeTab === "transactions"}
                <Transactions />
              {:else}
                {unreachable(activeTab)}
              {/if}
            </div>
          {:else}
            <LinkAddress />
          {/if}
        {:else}
          <WrongNetwork
            walletNetwork={w.connected.network}
            expectedNetwork={supportedNetwork($ethereumEnvironment)} />
        {/if}
      </div>
    {:else}
      <ConnectWallet
        onConnect={connectWallet}
        connecting={$wallet.status === Status.Connecting} />
    {/if}
  </div>
</ScreenLayout>
