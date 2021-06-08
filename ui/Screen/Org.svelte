<script lang="typescript">
  import * as router from "ui/src/router";
  import * as ipc from "ui/src/ipc";
  import * as notification from "ui/src/notification";
  import type * as theGraphApi from "ui/src/theGraphApi";

  import { Icon } from "ui/DesignSystem/Primitive";
  import {
    ActionBar,
    AdditionalActionsDropdown,
    FollowToggle,
    Header,
    SidebarLayout,
    TabBar,
  } from "ui/DesignSystem/Component";

  import ProjectsTab from "ui/Screen/Org/Projects.svelte";
  import MembersTab from "ui/Screen/Org/Members.svelte";
  import OrgHeader from "ui/Screen/Org/OrgHeader.svelte";
  import ProjectsMenu from "ui/Screen/Org/ProjectsMenu.svelte";
  import MembersMenu from "ui/Screen/Org/MembersMenu.svelte";

  export let activeTab: router.LoadedOrgTab;
  export let gnosisSafeAddress: string;
  export let address: string;
  export let members: theGraphApi.Member[];
  export let threshold: number;

  const tabs = (address: string, active: router.LoadedOrgTab) => {
    return [
      {
        title: "Anchored Projects",
        icon: Icon.ChevronLeftRight,
        active: active.type === "projects",
        onClick: () => {
          router.push({ type: "org", activeTab: "projects", address });
        },
      },
      {
        title: "Members",
        icon: Icon.User,
        active: active.type === "members",
        counter: members.length,
        onClick: () => {
          router.push({ type: "org", activeTab: "members", address });
        },
      },
    ];
  };

  const copy = (content: string): void => {
    if (content) ipc.copyToClipboard(content.trim());
    notification.info({ message: "Copied to your clipboard" });
  };

  const menuItems = (address: string, gnosisSafeAddress: string) => {
    return [
      {
        title: "Copy Org ID",
        icon: Icon.At,
        event: () => copy(address),
      },
      {
        title: "View on Gnosis Safe",
        icon: Icon.ArrowBoxUpRight,
        // TODO(rudolfs): make the link go to
        // `https://gnosis-safe.io/app/#/safes/${gnosisSafeAddress}` for
        // mainnet
        event: () => {
          window.location.href = `https://rinkeby.gnosis-safe.io/app/#/safes/${gnosisSafeAddress}`;
        },
      },
      {
        title: "Register ENS name",
        icon: Icon.Ethereum,
        disabled: true,
        event: () => {},
        tooltip: "Not yet supported",
      },
    ];
  };
</script>

<SidebarLayout>
  <Header>
    <OrgHeader
      slot="left"
      orgAddress={address}
      {gnosisSafeAddress}
      {threshold} />
    <div slot="right" style="display: flex">
      <FollowToggle following disabled style="margin-right: 1rem;" />
      <AdditionalActionsDropdown
        menuItems={menuItems(address, gnosisSafeAddress)} />
    </div>
  </Header>

  <ActionBar>
    <div slot="left">
      <TabBar tabs={tabs(address, activeTab)} />
    </div>
    <div slot="right">
      {#if activeTab.type === "projects"}
        <ProjectsMenu orgAddress={address} {gnosisSafeAddress} />
      {:else if activeTab.type === "members"}
        <MembersMenu {gnosisSafeAddress} />
      {:else}
        {router.unreachable(activeTab)}
      {/if}
    </div>
  </ActionBar>

  {#if activeTab.type === "projects"}
    <ProjectsTab
      {address}
      {gnosisSafeAddress}
      anchoredProjects={activeTab.anchoredProjects}
      unresolvedAnchors={activeTab.unresolvedAnchors} />
  {:else if activeTab.type === "members"}
    <MembersTab members={activeTab.members} />
  {:else}
    {router.unreachable(activeTab)}
  {/if}
</SidebarLayout>
