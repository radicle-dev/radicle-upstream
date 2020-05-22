<script>
  import { getContext } from "svelte";
  import Router, { link, push } from "svelte-spa-router";

  import * as path from "../src/path.ts";

  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    Topbar,
  } from "../DesignSystem/Component";
  import { Avatar, Icon } from "../DesignSystem/Primitive";

  import Screen from "../Layout/Screen.svelte";

  import Projects from "./Profile/Projects.svelte";
  import Wallet from "./Profile/Wallet.svelte";

  const screenRoutes = {
    "/profile/": Projects,
    "/profile/projects": Projects,
    "/profile/wallet": Wallet,
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
    {
      title: "Register handle",
      dataCy: "register-handle",
      icon: Icon.Register,
      event: () => push(path.registerUser()),
    },
  ];

  const session = getContext("session");
</script>

<Screen id="profile-screen" dataCy="profile-screen">
  <div slot="topbar">
    <Topbar>
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
  </div>

  <Router routes={screenRoutes} />
</Screen>
