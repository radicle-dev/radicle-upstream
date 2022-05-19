<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as orgRoute from "./OrgScreen/route";
  import type { Registration } from "ui/src/org/ensResolver";

  import * as router from "ui/src/router";
  import * as ipc from "ui/src/ipc";
  import * as org from "ui/src/org";
  import { unreachable } from "ui/src/unreachable";

  import AtIcon from "design-system/icons/At.svelte";
  import ChevronLeftRightIcon from "design-system/icons/ChevronLeftRight.svelte";
  import EthereumIcon from "design-system/icons/Ethereum.svelte";
  import GlobeIcon from "design-system/icons/Globe.svelte";
  import GnosisIcon from "design-system/icons/Gnosis.svelte";
  import UserIcon from "design-system/icons/User.svelte";

  import TrackToggle from "design-system/TrackToggle.svelte";
  import ThreeDotsMenu from "design-system/ThreeDotsMenu.svelte";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import ProjectsTab from "ui/App/OrgScreen/Projects.svelte";
  import MembersTab from "ui/App/OrgScreen/Members.svelte";
  import OrgHeader from "ui/App/OrgScreen/OrgHeader.svelte";
  import OrgSidebar from "ui/App/OrgScreen/OrgSidebar.svelte";
  import ProjectsMenu from "ui/App/OrgScreen/ProjectsMenu.svelte";
  import MembersMenu from "ui/App/OrgScreen/MembersMenu.svelte";

  export let activeTab: orgRoute.MultiSigView;
  export let gnosisSafeAddress: string;
  export let address: string;
  export let memberCount: number;
  export let threshold: number;
  export let registration: Registration | undefined = undefined;
  export let showWriteActions: boolean;

  const tabs = (address: string, active: orgRoute.MultiSigView) => {
    return [
      {
        title: "Anchored projects",
        icon: ChevronLeftRightIcon,
        active: active.type === "projects",
        onClick: () => {
          router.push({ type: "org", params: { view: "projects", address } });
        },
      },
      {
        title: "Members",
        icon: UserIcon,
        active: active.type === "members",
        counter: memberCount,
        onClick: () => {
          router.push({ type: "org", params: { view: "members", address } });
        },
      },
    ];
  };

  const menuItems = (address: string, gnosisSafeAddress: string) => {
    const items = [
      {
        title: "View on Etherscan",
        icon: AtIcon,
        event: () => {
          org.openOnEtherscan(address);
        },
      },
      {
        title: "View on Gnosis Safe",
        icon: GnosisIcon,
        event: () => {
          org.openOnGnosisSafe(gnosisSafeAddress, "transactions/queue");
        },
      },
      {
        title: "View in browser",
        icon: GlobeIcon,
        event: () => {
          ipc.openUrl(`https://app.radicle.network/orgs/${address}`);
        },
      },
    ];

    if (showWriteActions) {
      items.push({
        title: registration ? "Edit ENS name" : "Register ENS name",
        icon: EthereumIcon,
        event: () =>
          org.openEnsConfiguration(address, registration, gnosisSafeAddress),
      });
    }
    return items;
  };
</script>

<style>
  .sidebar-layout {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 1.5rem;
  }
  main {
    grid-column: 1 / span 2;
  }
</style>

<ScreenLayout>
  <div slot="header" style="display: flex; gap: 1rem">
    <OrgHeader {registration} orgAddress={address} />
    <div style="margin-left: auto; align-self: center; display: flex">
      <TrackToggle tracking disabled style="margin-right: 1rem;" />
      <ThreeDotsMenu menuItems={menuItems(address, gnosisSafeAddress)} />
    </div>
  </div>

  <div class="sidebar-layout">
    <main>
      <ActionBar style="padding: 0; margin-top: 1rem;">
        <TabBar tabs={tabs(address, activeTab)} />
        <div style="margin-left: auto" />
        {#if showWriteActions}
          {#if activeTab.type === "projects"}
            <ProjectsMenu
              isMultiSig={true}
              orgAddress={address}
              {gnosisSafeAddress}
              availableProjectCount={activeTab.projectCount}
              hasPendingAnchors={activeTab.anchors.pendingResolved.length !==
                0 || activeTab.anchors.pendingUnresolved.length !== 0} />
          {:else if activeTab.type === "members"}
            <MembersMenu {gnosisSafeAddress} />
          {:else}
            {unreachable(activeTab)}
          {/if}
        {/if}
      </ActionBar>

      {#if activeTab.type === "projects"}
        <ProjectsTab
          isMultiSig={true}
          {showWriteActions}
          {address}
          ownerAddress={gnosisSafeAddress}
          disableAnchorCreation={activeTab.projectCount === 0}
          anchors={activeTab.anchors} />
      {:else if activeTab.type === "members"}
        <MembersTab members={activeTab.members} />
      {:else}
        {unreachable(activeTab)}
      {/if}
    </main>
    <OrgSidebar
      {registration}
      {threshold}
      ownerAddress={gnosisSafeAddress}
      {memberCount} />
  </div>
</ScreenLayout>
