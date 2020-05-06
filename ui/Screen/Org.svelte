<script>
  import Router, { link, push } from "svelte-spa-router";
  import * as path from "../lib/path.js";
  import { orgMocks } from "../lib/orgMocks.js";
  import { Avatar, Icon } from "../DesignSystem/Primitive";

  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    SidebarLayout,
    Topbar
  } from "../DesignSystem/Component";

  import Projects from "./Org/Projects.svelte";
  import Fund from "./Org/Fund.svelte";
  import Members from "./Org/Members.svelte";

  export let params = null;

  /* TODO(rudolfs): replace with actual org metadata lookup */
  $: org = orgMocks.data.orgs.find(org => {
    return org.id === params.id;
  });

  const routes = {
    "/orgs/:id": Projects,
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
      event: () => push(path.registerProject(params.id))
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

<SidebarLayout
  style="margin-top: calc(var(--topbar-height) + 33px)"
  dataCy="page-container">
  <Topbar style="position: fixed; top: 0;">
    <a slot="left" href={path.orgProjects(params.id)} use:link>
      <Avatar
        title={org.metadata.name}
        imageUrl={org.metadata.avatarUrl}
        avatarFallback={org.avatarFallback}
        variant="square" />
    </a>

    <div slot="middle">
      <HorizontalMenu items={topbarMenuItems(params.id)} />
    </div>

    <div style="display: flex" slot="right">
      <Router routes={menuRoutes} />
      <AdditionalActionsDropdown
        style="margin: 0 24px 0 16px"
        headerTitle={params.id}
        menuItems={dropdownMenuItems} />
    </div>
  </Topbar>
  <Router {routes} />
</SidebarLayout>
