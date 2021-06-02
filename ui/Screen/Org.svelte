<script lang="typescript">
  import * as org from "ui/src/org";
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

  const orgScreenStore = org.orgScreenStore;

  export let activeTab: router.OrgTab;
  export let address: string;

  const tabs = (address: string, active: router.OrgTab) => {
    return [
      {
        title: "Projects",
        icon: Icon.ChevronLeftRight,
        active: active === "projects",
        onClick: () => {
          router.push({ type: "org", activeTab: "projects", address });
        },
      },
      {
        title: "Members",
        icon: Icon.User,
        active: active === "members",
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
    <OrgHeader
      slot="left"
      orgAddress={$orgScreenStore ? $orgScreenStore.orgAddress : ""}
      gnosisSafeAddress={$orgScreenStore
        ? $orgScreenStore.gnosisSafeAddress
        : ""} />
    <div slot="right" style="display: flex">
      <FollowToggle following disabled />
      <AdditionalActionsDropdown
        headerTitle={address}
        style="margin-left: 10px; border: 1px solid var(--color-foreground-level-3); border-radius: 4px;"
        menuItems={additionalActionsDropdownItems} />
    </div>
  </Header>

  <ActionBar>
    <div slot="left">
      <TabBar tabs={tabs(address, activeTab)} />
    </div>
    <div slot="right">
      {#if activeTab === "projects"}
        <ProjectsMenu />
      {:else if activeTab === "members"}
        <MembersMenu gnosisSafeAddress={$orgScreenStore?.gnosisSafeAddress} />
      {:else}
        {router.unreachable(activeTab)}
      {/if}
    </div>
  </ActionBar>

  {#if activeTab === "projects"}
    <ProjectsTab />
  {:else if activeTab === "members"}
    <MembersTab />
  {:else}
    {router.unreachable(activeTab)}
  {/if}
</SidebarLayout>
