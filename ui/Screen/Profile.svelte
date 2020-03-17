<script>
  import { SidebarLayout, Topbar } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";
  import Router from "svelte-spa-router";
  import * as path from "../lib/path.js";
  import {
    identityHandleStore,
    identityAvatarUrlStore
  } from "../store/identity.js";

  import Projects from "./Profile/Projects.svelte";
  import Wallet from "./Profile/Wallet.svelte";
  import Settings from "./Profile/Settings.svelte";
  import NotFound from "./NotFound.svelte";

  const topbarMenuItems = () => [
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

  const routes = {
    "/profile/": Projects,
    "/profile/projects": Projects,
    "/profile/wallet": Wallet,
    "/profile/settings": Settings,
    "*": NotFound
  };
</script>

<SidebarLayout
  style="margin-top: calc(var(--topbar-height) + 33px)"
  dataCy="page-container">
  <Topbar
    style="position: fixed; top: 0;"
    name={$identityHandleStore}
    avatarUrl={$identityAvatarUrlStore}
    href={path.profileProjects()}
    menuItems={topbarMenuItems()} />
  <Router {routes} />
</SidebarLayout>
