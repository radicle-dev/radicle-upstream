<script>
  import Router, { link, push } from "svelte-spa-router";

  import * as path from "../lib/path.js";
  import { session as store } from "../src/session.ts";

  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    Remote,
    SidebarLayout,
    Topbar
  } from "../DesignSystem/Component";
  import { Avatar, Icon } from "../DesignSystem/Primitive";

  import Onboard from "./Profile/Onboard.svelte";
  import Projects from "./Profile/Projects.svelte";
  import Wallet from "./Profile/Wallet.svelte";
  import Settings from "./Profile/Settings.svelte";
  import NotFound from "./NotFound.svelte";

  const screenRoutes = {
    "/profile/": Projects,
    "/profile/onboard/": Onboard,
    "/profile/projects": Projects,
    "/profile/wallet": Wallet,
    "/profile/settings": Settings,
    "*": NotFound
  };

  import ProjectsMenu from "./Profile/ProjectsMenu.svelte";
  import WalletMenu from "./Profile/WalletMenu.svelte";
  import SettingsMenu from "./Profile/SettingsMenu.svelte";

  const menuRoutes = {
    "/profile/projects": ProjectsMenu,
    "/profile/wallet": WalletMenu,
    "/profile/settings": SettingsMenu
  };

  const topbarMenuItems = [
    {
      icon: Icon.Source,
      title: "Projects",
      href: path.profileProjects(),
      looseActiveStateMatching: true
    },
    {
      icon: Icon.Fund,
      title: "Wallet",
      href: path.profileWallet(),
      looseActiveStateMatching: false
    },
    {
      icon: Icon.Settings,
      title: "Settings",
      href: path.profileSettings(),
      looseActiveStateMatching: false
    }
  ];

  const dropdownMenuItems = [
    {
      title: "New project",
      dataCy: "new-project",
      icon: Icon.Plus,
      event: () => push(path.createProject())
    },
    {
      title: "Register handle",
      dataCy: "register-handle",
      icon: Icon.Register,
      event: () => push(path.registerUser())
    }
  ];
</script>

<style>
  .profile-avatar {
    display: flex;
    align-items: center;
  }
</style>

<SidebarLayout
  style="margin-top: calc(var(--topbar-height) + 33px)"
  dataCy="profile-screen">

  <Remote {store} let:data={session}>
    <Topbar style="position: fixed; top: 0;">
      <a slot="left" href={path.profileProjects()} use:link>
        <!-- TODO(xla): Handle other states -->
        <div class="profile-avatar">
          <Avatar
            avatarFallback={session.identity.avatarFallback}
            imageUrl={session.identity.metadata.avatarUrl}
            variant="circle"
            title={session.identity.metadata.handle}
            size="regular"
            style="color: var(--color-secondary)" />
          {#if session.identity.registered}
            <Icon.Badge style="margin-left: 8px; fill: var(--color-primary);" />
          {/if}
        </div>
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
  </Remote>

  <Router routes={screenRoutes} />
</SidebarLayout>
