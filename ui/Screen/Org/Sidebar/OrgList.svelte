<script lang="typescript">
  import * as radicleAvatar from "radicle-avatar";
  import type { Identity } from "ui/src/identity";

  import { activeRouteStore, push } from "ui/src/router";

  import { orgSidebarStore } from "ui/src/org";
  import * as modal from "ui/src/modal";
  import * as wallet from "ui/src/wallet";

  import { Avatar } from "ui/DesignSystem/Primitive";
  import { Tooltip, SidebarItem } from "ui/DesignSystem/Component";
  import AddOrgButton from "./AddOrgButton.svelte";
  import ModalCreateOrg from "ui/Modal/Org/Create.svelte";

  export let identity: Identity;

  const walletStore = wallet.store;
  $: w = $walletStore;
</script>

{#if $w.status === wallet.Status.Connected}
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
            $w.status === wallet.Status.Connected
              ? $w.connected.account.address
              : null,
        })}>
      <AddOrgButton />
    </SidebarItem>
  </Tooltip>
{/if}
