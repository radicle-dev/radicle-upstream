<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Registration } from "ui/src/org/ensResolver";

  import * as ipc from "ui/src/ipc";
  import * as router from "ui/src/router";
  import * as org from "ui/src/org";

  import { FollowToggle, Icon, ThreeDotsMenu } from "ui/DesignSystem";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import ProjectsTab from "ui/App/OrgScreen/Projects.svelte";
  import OrgHeader from "ui/App/OrgScreen/OrgHeader.svelte";
  import OrgSidebar from "ui/App/OrgScreen/OrgSidebar.svelte";
  import ProjectsMenu from "ui/App/OrgScreen/ProjectsMenu.svelte";

  export let owner: string;
  export let address: string;
  export let projectCount: number;
  export let anchors: org.OrgAnchors;
  export let registration: Registration | undefined = undefined;
  export let showWriteActions: boolean;

  const tabs = (address: string) => {
    return [
      {
        title: "Anchored projects",
        icon: Icon.ChevronLeftRight,
        active: true,
        onClick: () => {
          router.push({
            type: "org",
            params: { address, view: "projects" },
          });
        },
      },
    ];
  };

  const menuItems = (address: string) => {
    return [
      {
        title: "View on Etherscan",
        icon: Icon.At,
        event: () => {
          org.openOnEtherscan(address);
        },
      },
      {
        title: "View in browser",
        icon: Icon.Globe,
        event: () => {
          ipc.openUrl(`https://app.radicle.network/orgs/${address}`);
        },
      },
      {
        title: registration ? "Edit ENS name" : "Register ENS name",
        icon: Icon.Ethereum,
        event: () => org.openEnsConfiguration(address, registration),
      },
    ];
  };

  const showSidebar: boolean = !!(
    registration?.url ||
    registration?.github ||
    registration?.twitter ||
    (registration?.seedId && registration?.seedHost)
  );
</script>

<style>
  .sidebar-layout {
    max-width: var(--content-max-width);
    margin: 0 auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
    display: grid;
    grid-template-columns: auto 18rem;
    gap: 1.5rem;
  }

  .single-column {
    grid-template-columns: 1fr;
  }
</style>

<ScreenLayout>
  <div slot="header" style="display: flex; gap: 1rem">
    <OrgHeader {registration} slot="left" orgAddress={address} />
    <div style="margin-left: auto; align-self: center; display: flex">
      <FollowToggle following disabled style="margin-right: 1rem;" />
      <ThreeDotsMenu menuItems={menuItems(address)} />
    </div>
  </div>
  <div class="sidebar-layout" class:single-column={!showSidebar}>
    <main>
      <ActionBar style="padding: 0; margin-top: 1rem;">
        <div slot="left">
          <TabBar tabs={tabs(address)} />
        </div>
        <div slot="right">
          {#if showWriteActions}
            <ProjectsMenu
              isMultiSig={false}
              orgAddress={address}
              gnosisSafeAddress={owner}
              availableProjectCount={projectCount}
              hasPendingAnchors={anchors.pendingResolved.length !== 0 ||
                anchors.pendingUnresolved.length !== 0} />
          {/if}
        </div>
      </ActionBar>

      <ProjectsTab
        isMultiSig={false}
        {address}
        ownerAddress={owner}
        {showWriteActions}
        disableAnchorCreation={projectCount === 0}
        {anchors} />
    </main>
    {#if showSidebar}
      <OrgSidebar {registration} ownerAddress={owner} />
    {/if}
  </div>
</ScreenLayout>
