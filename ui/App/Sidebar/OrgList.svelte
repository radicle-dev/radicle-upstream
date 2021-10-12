<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Identity } from "ui/src/identity";
  import { range } from "lodash";

  import { activeRouteStore, push } from "ui/src/router";

  import { orgSidebarStore, pendingOrgs } from "ui/src/org";
  import * as ethereum from "ui/src/ethereum";
  import * as modal from "ui/src/modal";
  import * as Wallet from "ui/src/wallet";

  const ethereumEnvironment = ethereum.selectedEnvironment;
  const walletStore = Wallet.store;

  import { Avatar, Icon, Tooltip, format } from "ui/DesignSystem";

  import CreateOrgModal from "ui/App/CreateOrgModal.svelte";
  import SidebarItem from "./SidebarItem.svelte";

  export let identity: Identity;

  $: wallet = $walletStore;
</script>

{#if $wallet.status === Wallet.Status.Connected && ethereum.supportedNetwork($ethereumEnvironment) === $wallet.connected.network}
  {#if $orgSidebarStore.type === "fetched" && $orgSidebarStore.orgs.length > 0}
    <Tooltip value="Loading orgs…">
      <SidebarItem>
        <Avatar size="regular" kind={{ type: "pendingOrg" }} />
      </SidebarItem>
    </Tooltip>
  {:else if $orgSidebarStore.type === "resolved"}
    {#each $orgSidebarStore.orgs as org (org.id)}
      <Tooltip
        value={org.registration?.domain || format.shortEthAddress(org.id)}>
        <SidebarItem
          indicator={true}
          onClick={() =>
            push({
              type: "org",
              params: { address: org.id, view: "projects" },
            })}
          active={($activeRouteStore.type === "singleSigOrg" ||
            $activeRouteStore.type === "multiSigOrg") &&
            $activeRouteStore.address === org.id}>
          <Avatar
            size="regular"
            kind={org.registration?.avatar
              ? { type: "orgImage", url: org.registration.avatar }
              : { type: "orgEmoji", uniqueIdentifier: org.id }} />
        </SidebarItem>
      </Tooltip>
    {/each}
  {/if}
  {#each range($pendingOrgs) as i (i)}
    <Tooltip value="Your org is being created">
      <SidebarItem>
        <Avatar size="regular" kind={{ type: "pendingOrg" }} />
      </SidebarItem>
    </Tooltip>
  {/each}
  <Tooltip value="Create an org">
    <SidebarItem
      indicator
      onClick={() =>
        modal.toggle(CreateOrgModal, () => {}, {
          identity,
          walletAddress:
            $wallet.status === Wallet.Status.Connected
              ? $wallet.connected.address
              : null,
        })}>
      <Icon.Plus />
    </SidebarItem>
  </Tooltip>
{:else}
  <Tooltip value="Create an org">
    <SidebarItem
      indicator
      onClick={() => {
        push({ type: "wallet", activeTab: "transactions" });
      }}>
      <Icon.Plus />
    </SidebarItem>
  </Tooltip>
{/if}
