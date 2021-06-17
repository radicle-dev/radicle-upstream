<script lang="typescript">
  import * as wallet from "ui/src/wallet";
  import * as transaction from "ui/src/transaction";
  import * as ethereum from "ui/src/ethereum";

  import { Icon } from "ui/DesignSystem/Primitive";
  import Tooltip from "ui/DesignSystem/Component/Tooltip.svelte";
  import SidebarItem from "./SidebarItem.svelte";

  const selectedEnvironment = ethereum.selectedEnvironment;
  const walletConnectionStore = wallet.store;
  const transactionStore = transaction.store;

  export let active: boolean;
  export let onClick: () => void;

  $: walletStore = $walletConnectionStore;

  let tooltipMessage: string;
  let iconConnected: boolean;
  let iconStatusColor: string | undefined;

  $: {
    if ($walletStore.status === wallet.Status.Connected) {
      const pendingTxs = $transactionStore.filter(
        tx => tx.status === transaction.TxStatus.AwaitingInclusion
      );

      const connectedNetwork = ethereum.supportedNetwork($selectedEnvironment);
      const walletNetwork = $walletStore.connected.network;

      iconConnected = true;

      if (connectedNetwork !== $walletStore.connected.network) {
        tooltipMessage = `Your wallet is on ${walletNetwork}, but Upstream is on ${connectedNetwork}`;
        iconStatusColor = "var(--color-negative)";
      } else if (pendingTxs.length > 0) {
        tooltipMessage = `${pendingTxs.length} pending transaction${
          pendingTxs.length > 1 ? "s" : ""
        }`;
        iconStatusColor = "var(--color-caution)";
      } else {
        if (walletStore.environment === ethereum.Environment.Mainnet) {
          tooltipMessage = "Connected";
        } else {
          tooltipMessage = `Connected to ${walletStore.environment}`;
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
    <Icon.Wallet connected={iconConnected} statusColor={iconStatusColor} />
  </SidebarItem>
</Tooltip>
