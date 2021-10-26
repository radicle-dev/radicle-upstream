<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as Wallet from "ui/src/wallet";
  import * as transaction from "ui/src/transaction";
  import * as ethereum from "ui/src/ethereum";

  import WalletIcon from "design-system/icons/Wallet.svelte";
  import Tooltip from "design-system/Tooltip.svelte";

  import SidebarItem from "./SidebarItem.svelte";

  const selectedEnvironment = ethereum.selectedEnvironment;
  const walletStore = Wallet.store;
  const transactionStore = transaction.store;

  export let active: boolean;
  export let onClick: () => void;

  $: wallet = $walletStore;

  let tooltipMessage: string;
  let iconConnected: boolean;
  let iconStatusColor: string | undefined;

  $: {
    if ($wallet.status === Wallet.Status.Connected) {
      const pendingTxs = $transactionStore.filter(
        tx => tx.status === transaction.TxStatus.AwaitingInclusion
      );

      const connectedNetwork = ethereum.supportedNetwork($selectedEnvironment);
      const walletNetwork = $wallet.connected.network;

      iconConnected = true;

      if (connectedNetwork !== $wallet.connected.network) {
        tooltipMessage = `Your wallet is on ${walletNetwork}, but Upstream is on ${connectedNetwork}`;
        iconStatusColor = "var(--color-negative)";
      } else if (pendingTxs.length > 0) {
        tooltipMessage = `${pendingTxs.length} pending transaction${
          pendingTxs.length > 1 ? "s" : ""
        }`;
        iconStatusColor = "var(--color-caution)";
      } else {
        if (wallet.environment === ethereum.Environment.Mainnet) {
          tooltipMessage = "Connected";
        } else {
          tooltipMessage = `Connected to ${wallet.environment}`;
        }
        iconStatusColor = undefined;
      }
    } else {
      tooltipMessage = "Not connected";
      iconConnected = false;
      iconStatusColor = undefined;
    }
  }
</script>

<Tooltip value={`Wallet • ${tooltipMessage}`}>
  <SidebarItem dataCy="wallet" indicator {active} onClick={() => onClick()}>
    <WalletIcon connected={iconConnected} statusColor={iconStatusColor} />
  </SidebarItem>
</Tooltip>
