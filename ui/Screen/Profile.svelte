<script>
  import { getContext } from "svelte";
  import Router, { push } from "svelte-spa-router";

  import * as path from "../src/path.ts";
  import { isDev } from "../../native/ipc.js";

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
      icon: Icon.ChevronLeftRight,
      title: "Projects",
      href: path.profileProjects(),
      looseActiveStateMatching: true,
    },
  ];

  if (isDev()) {
    topbarMenuItems.push({
      icon: Icon.Wallet,
      title: "Wallet",
      href: path.profileWallet(),
      looseActiveStateMatching: false,
    });
  }

  const dropdownMenuItems = [
    {
      title: "New project",
      dataCy: "new-project",
      icon: Icon.Plus,
      event: () => push(path.createProject()),
    },
  ];

  const session = getContext("session");
</script>

<SidebarLayout style="margin-top: 0;" dataCy="profile-screen">

  <Header.Large variant="profile" entity={session.identity}>
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
