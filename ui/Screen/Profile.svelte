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
  import Settings from "./Profile/Settings.svelte";
  import NotFound from "./NotFound.svelte";

  const screenRoutes = {
    "/profile/": Projects,
    "/profile/projects": Projects,
    "/profile/wallet": Wallet,
    "/profile/settings": Settings,
    "*": NotFound,
  };

  import ProjectsMenu from "./Profile/ProjectsMenu.svelte";
  import WalletMenu from "./Profile/WalletMenu.svelte";
  import SettingsMenu from "./Profile/SettingsMenu.svelte";

  const menuRoutes = {
    "/profile/projects": ProjectsMenu,
    "/profile/wallet": WalletMenu,
    "/profile/settings": SettingsMenu,
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
    {
      icon: Icon.Settings,
      title: "Settings",
      href: path.profileSettings(),
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
    {
      title: "Register handle",
      dataCy: "register-handle",
      icon: Icon.Register,
      event: () => push(path.registerUser()),
    },
  ];

  const session = getContext("session");
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
        imageUrl={session.identity.metadata.avatarUrl}
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
