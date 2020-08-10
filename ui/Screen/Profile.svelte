<script>
  import { getContext } from "svelte";
  import Router, { push } from "svelte-spa-router";

  import * as path from "../src/path.ts";

  import {
    AdditionalActionsDropdown,
    Header,
    HorizontalMenu,
    SidebarLayout,
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  import Projects from "./Profile/Projects.svelte";
  import Wallet from "./Profile/Wallet.svelte";
  import NotFound from "./NotFound.svelte";

  const screenRoutes = {
    "/profile/projects": Projects,
    "/profile/wallet": Wallet,
    "*": NotFound,
  };

  import ProjectsMenu from "./Profile/ProjectsMenu.svelte";

  const menuRoutes = {
    "/profile/projects": ProjectsMenu,
  };

  const topbarMenuItems = [
    {
      icon: Icon.Source,
      title: "Projects",
      href: path.profileProjects(),
      looseActiveStateMatching: true,
    },
    {
      icon: Icon.Fund,
      title: "Wallet",
      href: path.profileWallet(),
      looseActiveStateMatching: false,
    },
  ];

  const dropdownMenuItems = [
    {
      title: "New project",
      dataCy: "new-project",
      icon: Icon.Plus,
      event: () => push(path.createProject()),
    },
  ];

  const session = getContext("session");
  if (session.permissions.registerHandle) {
    dropdownMenuItems.push({
      title: "Register handle",
      dataCy: "register-handle",
      icon: Icon.Register,
      event: () => push(path.registerUser()),
    });
  }
</script>

<SidebarLayout style="margin-top: 0;" dataCy="profile-screen">

  <Header.Large
    variant="profile"
    entity={session.identity}
    on:registerHandle={() => push(path.registerUser())}>
    <div slot="left">
      <HorizontalMenu items={topbarMenuItems} />
    </div>
    <div slot="right" style="display: flex">
      <Router routes={menuRoutes} />
      <AdditionalActionsDropdown
        dataCy="profile-context-menu"
        style="margin: 0 16px"
        headerTitle={session.identity.shareableEntityIdentifier}
        menuItems={dropdownMenuItems} />
    </div>
  </Header.Large>

  <Router routes={screenRoutes} />
</SidebarLayout>
