<script>
  import { getContext } from "svelte";
  import Router, { push } from "svelte-spa-router";

  import { fetch, org as store } from "../src/org.ts";
  import * as path from "../src/path.ts";

  import { Icon } from "../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    Remote,
    BigHeader,
  } from "../DesignSystem/Component";

  import Sidebar from "../Layout/Sidebar.svelte";
  import Screen from "../Layout/Screen.svelte";

  import Fund from "./Org/Fund.svelte";
  import Members from "./Org/Members.svelte";
  import Projects from "./Org/Projects.svelte";

  import ProjectsMenu from "./Org/ProjectsMenu.svelte";
  import FundMenu from "./Org/FundMenu.svelte";
  import MembersMenu from "./Org/MembersMenu.svelte";

  export let params = null;

  const session = getContext("session");

  const routePrefix = "/orgs";
  const routes = {
    "/:id": Projects,
    "/:id/fund": Fund,
    "/:id/members": Members,
    "/:id/projects": Projects,
  };
  const menuRoutes = {
    "/:id/": ProjectsMenu,
    "/:id/projects": ProjectsMenu,
    "/:id/fund": FundMenu,
    "/:id/members": MembersMenu,
  };

  const topbarMenuItems = orgId => [
    {
      icon: Icon.Source,
      title: "Projects",
      href: path.orgProjects(orgId),
      looseActiveStateMatching: true,
    },
    {
      icon: Icon.Fund,
      title: "Fund",
      href: path.orgFund(orgId),
      looseActiveStateMatching: false,
    },
    {
      icon: Icon.Member,
      title: "Members",
      href: path.orgMembers(orgId),
      looseActiveStateMatching: false,
    },
  ];

  let registerProjectMenuItem;

  $: dropdownMenuItems = [
    registerProjectMenuItem,
    {
      title: "Add member",
      icon: Icon.Plus,
      event: () => push(path.memberRegistration(params.id)),
    },
    {
      title: "Send funds",
      icon: Icon.ArrowUp,
      event: () => console.log("event(send-funds-to-org)"),
    },
  ];

  if (session.permissions.registerProject) {
    registerProjectMenuItem = {
      dataCy: "add-project",
      title: "Add project",
      icon: Icon.Plus,
      event: () => push(path.registerProject(params.id)),
    };
  } else {
    registerProjectMenuItem = {
      dataCy: "add-project",
      title: "Add project",
      icon: Icon.Plus,
      disabled: true,
      tooltip: "To unlock project registration, create a local project first.",
    };
  }

  $: fetch({ id: params.id });
</script>

<Sidebar>
  <Screen dataCy="org-screen">
    <Remote {store} let:data={org}>
      <BigHeader variant="org" entity={org}>
        <div slot="left">
          <HorizontalMenu items={topbarMenuItems(params.id)} />
        </div>
        <div slot="right" style="display: flex">
          <Router prefix={routePrefix} routes={menuRoutes} />
          <AdditionalActionsDropdown
            dataCy="context-menu"
            style="margin: 0 16px"
            headerTitle={org.shareableEntityIdentifier}
            menuItems={dropdownMenuItems} />
        </div>
      </BigHeader>
      <Router prefix={routePrefix} {routes} />
    </Remote>
  </Screen>
</Sidebar>
