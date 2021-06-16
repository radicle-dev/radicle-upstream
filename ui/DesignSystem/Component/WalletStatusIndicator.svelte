<script lang="typescript">
  import { store, Status } from "ui/src/wallet";
  import { store as transactions } from "ui/src/transaction";

  import { Icon } from "../Primitive";
  import Tooltip from "./Tooltip.svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import * as ethereum from "ui/src/ethereum";

  export let active: boolean;

  export let onClick: () => void;

  $: pendingTxs = $transactions.filter(
    tx => tx.status === "Awaiting inclusion"
  );
  $: wallet = $store;

  const tooltipMessage = (environment: ethereum.Environment): string => {
    switch (environment) {
      case ethereum.Environment.Mainnet:
        return "Wallet · Connected";
      default:
        return `Wallet · Connected to ${environment}`;
    }
  };
</script>

<div>
  {#if $wallet.status === Status.Connected}
    {#if pendingTxs.length > 0}
      <Tooltip
        value={`Wallet · ${pendingTxs.length} pending transaction${
          pendingTxs.length > 1 ? "s" : ""
        }`}>
        <SidebarItem
          dataCy="wallet"
          indicator
          {active}
          onClick={() => onClick()}>
          <Icon.Wallet connected statusColor="var(--color-caution)" />
        </SidebarItem>
      </Tooltip>
    {:else}
      <Tooltip value={tooltipMessage(wallet.environment)}>
        <SidebarItem
          dataCy="wallet"
          indicator
          {active}
          onClick={() => onClick()}>
          <Icon.Wallet connected />
        </SidebarItem>
      </Tooltip>
    {/if}
  {:else}
    <Tooltip value="Wallet · Not connected">
      <SidebarItem dataCy="wallet" indicator {active} onClick={() => onClick()}>
        <Icon.Wallet />
      </SidebarItem>
    </Tooltip>
  {/if}
</div>
