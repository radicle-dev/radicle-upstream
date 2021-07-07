<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as radicleAvatar from "radicle-avatar";
  import type { Identity } from "ui/src/identity";
  import { range } from "lodash";

  import { activeRouteStore, push } from "ui/src/router";

  import { orgSidebarStore, pendingOrgs } from "ui/src/org";
  import * as modal from "ui/src/modal";
  import * as Wallet from "ui/src/wallet";
  import * as ethereum from "ui/src/ethereum";

  const ethereumEnvironment = ethereum.selectedEnvironment;
  const walletStore = Wallet.store;

  import Avatar from "ui/DesignSystem/Avatar.svelte";
  import Tooltip from "ui/DesignSystem/Tooltip.svelte";

  import ModalCreateOrg from "ui/Modal/Org/Create.svelte";
  import SidebarItem from "./SidebarItem.svelte";

  import Icon from "ui/DesignSystem/Icon";

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
        active={($activeRouteStore.type === "singleSigOrg" ||
          $activeRouteStore.type === "multiSigOrg") &&
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
  {#each range($pendingOrgs) as i (i)}
    <Tooltip value="Your org is being created">
      <SidebarItem>
        <Avatar
          size="regular"
          variant="square"
          animate={true}
          avatarFallback={{
            background: {
              r: 51,
              g: 62,
              b: 71,
            },
            emoji: "",
          }} />
      </SidebarItem>
    </Tooltip>
  {/each}
  <Tooltip value="Create an org">
    <SidebarItem
      indicator
      onClick={() =>
        modal.toggle(ModalCreateOrg, () => {}, {
          identity,
          walletAddress:
            $wallet.status === Wallet.Status.Connected
              ? $wallet.connected.account.address
              : null,
        })}>
      <Icon.Plus />
    </SidebarItem>
  </Tooltip>
{/if}
