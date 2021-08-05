<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Registration } from "ui/src/org/ensResolver";

  import * as ipc from "ui/src/ipc";
  import * as notification from "ui/src/notification";
  import * as router from "ui/src/router";
  import * as org from "ui/src/org";

  import {
    ActionBar,
    ThreeDotsMenu,
    FollowToggle,
    Header,
    Icon,
    SidebarLayout,
    TabBar,
  } from "ui/DesignSystem";

  import ProjectsTab from "ui/Screen/Org/Projects.svelte";
  import OrgHeader from "ui/Screen/Org/OrgHeader.svelte";
  import ProjectsMenu from "ui/Screen/Org/ProjectsMenu.svelte";

  export let owner: string;
  export let address: string;
  export let projectCount: number;
  export let anchors: org.OrgAnchors;
  export let registration: Registration | undefined = undefined;

  const tabs = (address: string) => {
    return [
      {
        title: "Anchored Projects",
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
        title: "Copy Org ID",
        icon: Icon.At,
        event: () => {
          ipc.copyToClipboard(address.trim());
          notification.info({ message: "Copied to your clipboard" });
        },
      },
      {
        title: "View in browser",
        icon: Icon.Globe,
        event: () => {
          ipc.openUrl(`https://app.radicle.network/orgs/${address}`);
        },
      },
      org.ensMenuItem(address, registration),
    ];
  };
</script>

<SidebarLayout>
  <Header>
    <OrgHeader
      {registration}
      slot="left"
      orgAddress={address}
      ownerAddress={owner} />
    <div slot="right" style="display: flex">
      <FollowToggle following disabled style="margin-right: 1rem;" />
      <ThreeDotsMenu menuItems={menuItems(address)} />
    </div>
  </Header>

  <ActionBar>
    <div slot="left">
      <TabBar tabs={tabs(address)} />
    </div>
    <div slot="right">
      <ProjectsMenu
        isMultiSig={false}
        orgAddress={address}
        gnosisSafeAddress={owner}
        availableProjectCount={projectCount}
        hasPendingAnchors={anchors.pendingResolved.length !== 0 ||
          anchors.pendingUnresolved.length !== 0} />
    </div>
  </ActionBar>

  <ProjectsTab
    isMultiSig={false}
    {address}
    ownerAddress={owner}
    disableAnchorCreation={projectCount === 0}
    {anchors} />
</SidebarLayout>
