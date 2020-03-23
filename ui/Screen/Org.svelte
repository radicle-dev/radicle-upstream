<script>
  import Router, { link } from "svelte-spa-router";
  import { setContext } from "svelte";
  import * as path from "../lib/path.js";
  import { Icon } from "../DesignSystem/Primitive";

  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    SidebarLayout,
    Topbar,
    IdentityAvatar
  } from "../DesignSystem/Component";

  import Projects from "./Org/Projects.svelte";
  import Fund from "./Org/Fund.svelte";
  import Members from "./Org/Members.svelte";

  export let params = null;

  setContext("orgId", params.id);

  const routes = {
    "/orgs/:id/": Projects,
    "/orgs/:id/projects": Projects,
    "/orgs/:id/fund": Fund,
    "/orgs/:id/members": Members
  };

  import ProjectsMenu from "./Org/ProjectsMenu.svelte";
  import FundMenu from "./Org/FundMenu.svelte";
  import MembersMenu from "./Org/MembersMenu.svelte";

  const menuRoutes = {
    "/orgs/:id/": ProjectsMenu,
    "/orgs/:id/projects": ProjectsMenu,
    "/orgs/:id/fund": FundMenu,
    "/orgs/:id/members": MembersMenu
  };

  const topbarMenuItems = orgId => [
    {
      icon: Icon.Source,
      title: "Projects",
      href: path.orgProjects(orgId),
      looseActiveStateMatching: true
    },
    {
      icon: Icon.Fund,
      title: "Fund",
      href: path.orgFund(orgId),
      looseActiveStateMatching: false
    },
    {
      icon: Icon.Member,
      title: "Members",
      href: path.orgMembers(orgId),
      looseActiveStateMatching: false
    }
  ];

  const dropdownMenuItems = [
    {
      title: "Add project",
      icon: Icon.Plus,
      event: () => console.log("event(add-project-to-org)")
    },
    {
      title: "Add member",
      icon: Icon.Plus,
      event: () => console.log("event(add-member-to-org)")
    },
    {
      title: "Send funds",
      icon: Icon.ArrowUp,
      event: () => console.log("event(send-funds-to-org)")
    }
  ];
</script>

<style>
  .name {
    display: flex;
    align-items: center;
    height: 100%;
    border-right: 1px solid var(--color-lightgray);
    padding-left: 16px;
    padding-right: 24px;
  }

  .right {
    display: flex;
    align-items: center;
    width: 100%;
    justify-content: flex-end;
  }
</style>

<SidebarLayout
  style="margin-top: calc(var(--topbar-height) + 33px)"
  dataCy="page-container">
  <Topbar style="position: fixed; top: 0;">
    <a slot="left" class="name" href={path.profileProjects()} use:link>
      <IdentityAvatar
        showTitle={true}
        size={'regular'}
        style="color: var(--color-purple)" />
    </a>

    <div slot="middle">
      <HorizontalMenu items={topbarMenuItems()} />
    </div>

    <div slot="right" class="right">
      <Router routes={menuRoutes} />
      <AdditionalActionsDropdown
        style="margin: 0 24px 0 16px"
        headerTitle={params.id}
        menuItems={dropdownMenuItems} />
    </div>
  </Topbar>
  <Router {routes} />
</SidebarLayout>
