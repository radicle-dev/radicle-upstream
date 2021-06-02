<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import { store, Status } from "ui/src/wallet";
  import { store as transactions } from "ui/src/transaction";

  import { Icon } from "../Primitive";
  import Tooltip from "./Tooltip.svelte";
  import SidebarItem from "./SidebarItem.svelte";

  export let active: boolean;

  const dispatch = createEventDispatcher();
  const onClick = () => {
    dispatch("walletClick");
  };

  $: pendingTxs = $transactions.filter(
    tx => tx.status === "Awaiting inclusion"
  );
  $: wallet = $store;
  // Hack to have Svelte working with checking the $wallet variant
  // and thus be able to access its appropriate fields.
  $: w = $wallet;
</script>

<div>
  {#if w.status === Status.Connected}
    <Tooltip value="Wallet · Connected">
      <SidebarItem
        dataCy="wallet"
        indicator
        {active}
        on:itemClick={() => onClick()}>
        <Icon.WalletStatus statusColor="var(--color-positive)" />
      </SidebarItem>
    </Tooltip>
  {:else if w.status === Status.Connected && pendingTxs.length > 0}
    <Tooltip value={`Wallet · ${pendingTxs.length} pending transactions`}>
      <SidebarItem
        dataCy="wallet"
        indicator
        {active}
        on:itemClick={() => onClick()}>
        <Icon.WalletStatus statusColor="var(--color-caution)" />
      </SidebarItem>
    </Tooltip>
  {:else}
    <Tooltip value="Wallet · Not connected">
      <SidebarItem
        dataCy="wallet"
        indicator
        {active}
        on:itemClick={() => onClick()}>
        <Icon.Wallet />
      </SidebarItem>
    </Tooltip>
  {/if}
</div>
