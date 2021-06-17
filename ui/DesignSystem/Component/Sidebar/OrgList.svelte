<script lang="typescript">
  import * as radicleAvatar from "radicle-avatar";
  import type { Identity } from "ui/src/identity";

  import { activeRouteStore, push } from "ui/src/router";

  import { orgSidebarStore } from "ui/src/org";
  import * as modal from "ui/src/modal";
  import * as Wallet from "ui/src/wallet";
  import * as ethereum from "ui/src/ethereum";

  const ethereumEnvironment = ethereum.selectedEnvironment;
  const walletStore = Wallet.store;

  import { Avatar } from "ui/DesignSystem/Primitive";
  import { Tooltip } from "ui/DesignSystem/Component";

  import ModalCreateOrg from "ui/Modal/Org/Create.svelte";
  import AddOrgButton from "./AddOrgButton.svelte";
  import SidebarItem from "./SidebarItem.svelte";

  export let identity: Identity;

  $: wallet = $walletStore;
</script>

{#if $wallet.status === Wallet.Status.Connected && ethereum.supportedNetwork($ethereumEnvironment) === $wallet.connected.network}
  {#each $orgSidebarStore as org (org.id)}
    <Tooltip value={org.id}>
      <SidebarItem
        indicator={true}
        onClick={() =>
          push({ type: "org", address: org.id, activeTab: "projects" })}
        active={$activeRouteStore.type === "org" &&
          $activeRouteStore.address === org.id}>
        <Avatar
          size="regular"
          variant="square"
          avatarFallback={radicleAvatar.generate(
            org.id,
            radicleAvatar.Usage.Any
          )} />
      </SidebarItem>
    </Tooltip>
  {/each}
  <Tooltip value="Create an org">
    <SidebarItem
      onClick={() =>
        modal.toggle(ModalCreateOrg, () => {}, {
          identity,
          walletAddress:
            $wallet.status === Wallet.Status.Connected
              ? $wallet.connected.account.address
              : null,
        })}>
      <AddOrgButton />
    </SidebarItem>
  </Tooltip>
{/if}
