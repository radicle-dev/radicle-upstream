<script lang="typescript">
  import { store, Status as WalletStatus } from "ui/src/wallet";
  import { store as transactions, TxStatus } from "ui/src/transaction";

  import { Icon } from "../Primitive";
  import Tooltip from "./Tooltip.svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import * as ethereum from "ui/src/ethereum";

  export let active: boolean;

  export let onClick: () => void;

  $: wallet = $store;

  let tooltipMessage: string;
  let iconConnected: boolean;
  let iconStatusColor: string | undefined;

  $: {
    if ($wallet.status === WalletStatus.Connected) {
      const pendingTxs = $transactions.filter(
        tx => tx.status === TxStatus.AwaitingInclusion
      );

      if (pendingTxs.length > 0) {
        tooltipMessage = `Wallet · ${pendingTxs.length} pending transaction${
          pendingTxs.length > 1 ? "s" : ""
        }`;
        iconConnected = true;
        iconStatusColor = "var(--color-caution)";
      } else {
        switch (wallet.environment) {
          case ethereum.Environment.Mainnet:
            tooltipMessage = "Wallet · Connected";
            break;
          default:
            tooltipMessage = `Wallet · Connected to ${wallet.environment}`;
            break;
        }
        iconConnected = true;
        iconStatusColor = undefined;
      }
    } else {
      tooltipMessage = "Wallet · Not connected";
      iconConnected = false;
      iconStatusColor = undefined;
    }
  }
</script>

<Tooltip value={tooltipMessage}>
  <SidebarItem dataCy="wallet" indicator {active} onClick={() => onClick()}>
    <Icon.Wallet connected={iconConnected} statusColor={iconStatusColor} />
  </SidebarItem>
</Tooltip>
