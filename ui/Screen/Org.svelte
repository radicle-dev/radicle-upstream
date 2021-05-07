<script lang="typescript">
  import Router from "svelte-spa-router";
  import { wrap } from "svelte-spa-router/wrap";

  import { orgScreenStore } from "../src/org";
  import * as org from "../src/org";
  import * as path from "../src/path";

  import { Icon } from "../DesignSystem/Primitive";
  import {
    ActionBar,
    AdditionalActionsDropdown,
    FollowToggle,
    Header,
    HorizontalMenu,
    SidebarLayout,
  } from "../DesignSystem/Component";

  import Projects from "./Org/Projects.svelte";
  import Members from "./Org/Members.svelte";
  import OrgHeader from "./Org/OrgHeader.svelte";
  import ProjectsMenu from "./Org/ProjectsMenu.svelte";
  import MembersMenu from "./Org/MembersMenu.svelte";

  export let params: { address: string };

  const membersWrap = wrap({
    component: Members,
    conditions: [
      async () => {
        try {
          await org.fetchMembers($orgScreenStore.gnosisSafeAddress);
          return true;
        } catch {
          return false;
        }
      },
    ],
  });

  const screenRoutes = {
    "/org/:address/projects": Projects,
    "/org/:address/members": membersWrap,
    "*": Projects,
  };

  $: topbarMenuItems = [
    {
      icon: Icon.ChevronLeftRight,
      title: "Projects",
      href: path.orgProjects(params.address),
    },
    {
      icon: Icon.User,
      title: "Members",
      href: path.orgMembers(params.address),
    },
  ];

  const menuRoutes = {
    "/org/:address/projects": ProjectsMenu,
    "/org/:address/members": wrap({
      component: MembersMenu,
      props: { gnosisSafeAddress: $orgScreenStore.gnosisSafeAddress },
    }),
    "*": ProjectsMenu,
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
      gnosisSafeWalletAddress={$orgScreenStore.orgAddress}
      gnosisSafeAddress={$orgScreenStore.gnosisSafeAddress} />
    <div slot="right" style="display: flex">
      <FollowToggle following disabled />
      <AdditionalActionsDropdown
        headerTitle={params.address}
        style="margin-left: 10px; border: 1px solid var(--color-foreground-level-3); border-radius: 4px;"
        menuItems={additionalActionsDropdownItems} />
    </div>
  </Header>

  <ActionBar>
    <div slot="left">
      <HorizontalMenu items={topbarMenuItems} />
    </div>
    <div slot="right">
      <Router routes={menuRoutes} />
    </div>
  </ActionBar>

  <Router routes={screenRoutes} />
</SidebarLayout>
