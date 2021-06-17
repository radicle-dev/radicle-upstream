<script lang="typescript">
  import * as wallet from "ui/src/wallet";
  import * as transaction from "ui/src/transaction";
  import * as ethereum from "ui/src/ethereum";

  import { Icon } from "../Primitive";
  import Tooltip from "./Tooltip.svelte";
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

      if (pendingTxs.length > 0) {
        tooltipMessage = `Wallet · ${pendingTxs.length} pending transaction${
          pendingTxs.length > 1 ? "s" : ""
        }`;
        iconConnected = true;
        iconStatusColor = "var(--color-caution)";
      } else {
        if (walletStore.environment === ethereum.Environment.Mainnet) {
          tooltipMessage = "Wallet · Connected";
        } else {
          tooltipMessage = `Wallet · Connected to ${walletStore.environment}`;
        }
        iconConnected = true;
        iconStatusColor = undefined;
      }
    } else {
      tooltipMessage = "Wallet · Not connected";
      iconConnected = false;
      iconStatusColor = undefined;
    }

    if ($walletStore.status === wallet.Status.Connected) {
      const connectedNetwork = ethereum.supportedNetwork($selectedEnvironment);
      const walletNetowrk = $walletStore.connected.network;

      if (connectedNetwork !== $walletStore.connected.network) {
        tooltipMessage =
          `Wallet · Your wallet is on ${walletNetowrk}, but Upstream is on ` +
          `${connectedNetwork}`;
        iconConnected = true;
        iconStatusColor = "var(--color-negative)";
      }
    }
  }
</script>

<Tooltip value={tooltipMessage}>
  <SidebarItem dataCy="wallet" indicator {active} onClick={() => onClick()}>
    <Icon.Wallet connected={iconConnected} statusColor={iconStatusColor} />
  </SidebarItem>
</Tooltip>
