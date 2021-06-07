<script lang="typescript">
  import * as router from "ui/src/router";

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

  const tabs = (address: string, active: router.LoadedOrgTab) => {
    return [
      {
        title: "Projects",
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
        onClick: () => {
          router.push({ type: "org", activeTab: "members", address });
        },
      },
    ];
  };

  const additionalActionsDropdownItems = [
    {
      title: "Change avatar",
      icon: Icon.Pen,
      event: () => console.log("event(Change avatar)"),
    },
  ];
</script>

<SidebarLayout>
  <Header>
    <OrgHeader slot="left" orgAddress={address} {gnosisSafeAddress} />
    <div slot="right" style="display: flex">
      <FollowToggle following disabled />
      <AdditionalActionsDropdown
        headerTitle={address}
        style="margin-left: 10px;"
        menuItems={additionalActionsDropdownItems} />
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
    <MembersTab
      {gnosisSafeAddress}
      members={activeTab.members}
      threshold={activeTab.threshold} />
  {:else}
    {router.unreachable(activeTab)}
  {/if}
</SidebarLayout>
