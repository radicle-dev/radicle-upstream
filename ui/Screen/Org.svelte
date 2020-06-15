<script>
  import Router, { push } from "svelte-spa-router";

  import { fetch, org as store } from "../src/org.ts";
  import * as path from "../src/path.ts";

  import { Icon } from "../DesignSystem/Primitive";
  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    Remote,
    SidebarLayout,
    BigHeader,
  } from "../DesignSystem/Component";

  import Fund from "./Org/Fund.svelte";
  import Members from "./Org/Members.svelte";
  import Projects from "./Org/Projects.svelte";

  export let params = null;

  const routes = {
    "/orgs/:id": Projects,
    "/orgs/:id/fund": Fund,
    "/orgs/:id/members": Members,
    "/orgs/:id/projects": Projects,
  };

  import ProjectsMenu from "./Org/ProjectsMenu.svelte";
  import FundMenu from "./Org/FundMenu.svelte";
  import MembersMenu from "./Org/MembersMenu.svelte";

  const menuRoutes = {
    "/orgs/:id/": ProjectsMenu,
    "/orgs/:id/projects": ProjectsMenu,
    "/orgs/:id/fund": FundMenu,
    "/orgs/:id/members": MembersMenu,
  };

  const topbarMenuItems = (orgId) => [
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

  const dropdownMenuItems = [
    {
      dataCy: "add-project",
      title: "Add project",
      icon: Icon.Plus,
      event: () => push(path.registerProject(params.id)),
    },
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

  fetch({ id: params.id });
</script>

<SidebarLayout dataCy="org-screen" style="margin-top: var(--bigheader-height);">
  <Remote {store} let:data={org}>
    <BigHeader variant="org" data={org} style="position: fixed; top: 0;">
      <div slot="left">
        <HorizontalMenu items={topbarMenuItems(params.id)} />
      </div>
      <div slot="right" style="display: flex">
        <Router routes={menuRoutes} />
        <AdditionalActionsDropdown
          dataCy="context-menu"
          style="margin: 0 16px"
          headerTitle={org.shareableEntityIdentifier}
          menuItems={dropdownMenuItems} />
      </div>
    </BigHeader>
    <Router {routes} />
  </Remote>
</SidebarLayout>
