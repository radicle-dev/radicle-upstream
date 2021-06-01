<script lang="typescript">
  import Router from "svelte-spa-router";

  import * as path from "ui/src/path";
  import { isDev } from "ui/src/config";

  import {
    selectedEnvironment as ethereumEnvironment,
    supportedNetwork,
  } from "ui/src/ethereum";
  import {
    watchAttestationStatus,
    attestationStatus,
    AttestationStatus,
  } from "ui/src/attestation/status";
  import { store, Status } from "ui/src/wallet";

  import ConnectWallet from "ui/DesignSystem/Component/Wallet/Connect.svelte";
  import WalletPanel from "ui/DesignSystem/Component/Wallet/Panel.svelte";
  import WrongNetwork from "ui/DesignSystem/Component/Wallet/WrongNetwork.svelte";

  import Pool from "./Wallet/Pool.svelte";
  import Transactions from "./Wallet/Transactions.svelte";
  import LinkAddress from "./Wallet/LinkAddress.svelte";

  import {
    EmptyState,
    HorizontalMenu,
    SidebarLayout,
  } from "ui/DesignSystem/Component";
  import { Icon } from "ui/DesignSystem/Primitive";

  const topbarMenuItems = [
    {
      icon: Icon.Transactions,
      title: "Transactions",
      href: path.walletTransactions(),
    },
  ];

  if (isDev) {
    topbarMenuItems.push({
      icon: Icon.TokenStreams,
      title: "Token Streams",
      href: path.walletStreams(),
    });
  }

  watchAttestationStatus(store);

  $: wallet = $store;
  // Hack to have Svelte working with checking the $wallet variant
  // and thus be able to access its appropriate fields.
  $: w = $wallet;

  const screenRoutes = {
    "/wallet/transactions": Transactions,
    "/wallet/streams": Pool,
  };
</script>

<style>
  .container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: var(--content-padding);
    margin: 0 auto;
  }
  .content {
    display: grid;
    grid-template-columns: 20rem auto;
    margin-top: 1.5rem;
    gap: 1.5rem;
  }

  .title {
    padding-left: 0.75rem;
    margin-bottom: 1rem;
  }
</style>

<SidebarLayout>
  <div class="container" data-cy="wallet">
    {#if w.status === Status.Connected}
      <div class="content">
        <div>
          <h1 class="title">Wallet</h1>
          <WalletPanel
            onDisconnect={wallet.disconnect}
            account={w.connected.account} />
        </div>
        {#if supportedNetwork($ethereumEnvironment) === w.connected.network}
          {#if $attestationStatus === AttestationStatus.Fetching}
            <EmptyState
              text="Checking whether you have attested your Ethereum address..."
              style="height: 30rem; margin-top: 3.75rem;"
              emoji="🧦" />
          {:else if $attestationStatus === AttestationStatus.Valid}
            <div class="right-column">
              <HorizontalMenu
                slot="left"
                items={topbarMenuItems}
                style="padding: 0.5rem 0; margin-bottom: 1rem;" />
              <Router routes={screenRoutes} />
            </div>
          {:else}
            <LinkAddress />
          {/if}
        {:else}
          <WrongNetwork
            expectedNetwork={supportedNetwork($ethereumEnvironment)} />
        {/if}
      </div>
    {:else}
      <ConnectWallet
        onConnect={wallet.connect}
        connecting={$wallet.status === Status.Connecting} />
    {/if}
  </div>
</SidebarLayout>