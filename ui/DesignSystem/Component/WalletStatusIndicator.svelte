<script lang="typescript">
  import { store, Status } from "ui/src/wallet";
  import { store as transactions } from "ui/src/transaction";

  import { Icon } from "../Primitive";
  import Tooltip from "./Tooltip.svelte";
  import SidebarItem from "./SidebarItem.svelte";

  export let active: boolean;

  export let onClick: () => void;

  $: pendingTxs = $transactions.filter(
    tx => tx.status === "Awaiting inclusion"
  );
  $: wallet = $store;
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
          <Icon.WalletStatus statusColor="var(--color-caution)" />
        </SidebarItem>
      </Tooltip>
    {:else}
      <Tooltip value="Wallet · Connected">
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
