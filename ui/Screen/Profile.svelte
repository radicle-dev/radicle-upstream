<script>
  import { getContext } from "svelte";
  import Router, { link, push } from "svelte-spa-router";

  import * as path from "../src/path.ts";

  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    SidebarLayout,
    Topbar,
  } from "../DesignSystem/Component";
  import { Avatar, Icon } from "../DesignSystem/Primitive";

  import Projects from "./Profile/Projects.svelte";
  import Wallet from "./Profile/Wallet.svelte";
  import NotFound from "./NotFound.svelte";

  const screenRoutes = {
    "/profile/": Projects,
    "/profile/projects": Projects,
    "/profile/wallet": Wallet,
    "*": NotFound,
  };

  import ProjectsMenu from "./Profile/ProjectsMenu.svelte";
  import WalletMenu from "./Profile/WalletMenu.svelte";

  const menuRoutes = {
    "/profile/projects": ProjectsMenu,
    "/profile/wallet": WalletMenu,
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

<SidebarLayout
  style="margin-top: calc(var(--topbar-height) + 33px)"
  dataCy="profile-screen">

  <Topbar style="position: fixed; top: 0;">
    <a slot="left" href={path.profileProjects()} use:link>
      <!-- TODO(xla): Handle other states -->
      <Avatar
        dataCy="profile-avatar"
        avatarFallback={session.identity.avatarFallback}
        variant="circle"
        title={session.identity.metadata.handle}
        size="regular"
        registered={session.identity.registered}
        style="color: var(--color-secondary)" />
    </a>
    <div slot="middle">
      <HorizontalMenu items={topbarMenuItems} />
    </div>
    <div slot="right" style="display: flex">
      <Router routes={menuRoutes} />
      <AdditionalActionsDropdown
        dataCy="profile-context-menu"
        style="margin: 0 24px 0 16px"
        headerTitle={session.identity.shareableEntityIdentifier}
        menuItems={dropdownMenuItems} />
    </div>
  </Topbar>

  <Router routes={screenRoutes} />
</SidebarLayout>
