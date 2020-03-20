<script>
  import { link, push } from "svelte-spa-router";
  import {
    AdditionalActionsDropdown,
    HorizontalMenu,
    SidebarLayout,
    Topbar,
    IdentityAvatar
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";
  import Router from "svelte-spa-router";
  import * as path from "../lib/path.js";

  import Projects from "./Profile/Projects.svelte";
  import Wallet from "./Profile/Wallet.svelte";
  import Settings from "./Profile/Settings.svelte";
  import NotFound from "./NotFound.svelte";

  const screenRoutes = {
    "/profile/": Projects,
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
      icon: Icon.Plus,
      event: () => push(path.createProject())
    },
    {
      title: "Register handle",
      icon: Icon.Register,
      event: () => push(path.registerUser())
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
      <HorizontalMenu items={topbarMenuItems} />
    </div>

    <div slot="right" class="right">
      <Router routes={menuRoutes} />
      <AdditionalActionsDropdown
        style="margin: 0 24px 0 16px"
        headerTitle="cloudhead@ViJQHAdeZoiEbaE5vv83dpjEun.rad"
        menuItems={dropdownMenuItems} />
    </div>
  </Topbar>
  <Router routes={screenRoutes} />
</SidebarLayout>
