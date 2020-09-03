<script>
  import { getContext } from "svelte";
  import Router from "svelte-spa-router";

  import * as path from "../src/path.ts";

  import {
    Header,
    HorizontalMenu,
    SidebarLayout,
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  import Projects from "./Profile/Projects.svelte";
  import Tracking from "./Profile/Tracking.svelte";
  import Wallet from "./Profile/Wallet.svelte";
  import NotFound from "./NotFound.svelte";

  const screenRoutes = {
    "/profile/projects": Projects,
    "/profile/wallet": Wallet,
    "/profile/tracking": Tracking,
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
    {
      icon: Icon.Network,
      title: "Following",
      href: path.profileTracking(),
      looseActiveStateMatching: false,
    },
  ];

  const session = getContext("session");
</script>

<SidebarLayout style="margin-top: 0;" dataCy="profile-screen">
  <Header.Large variant="profile" entity={session.identity}>
    <div slot="left">
      <HorizontalMenu items={topbarMenuItems} />
    </div>
    <div slot="top">
      <Router routes={menuRoutes} />
    </div>
  </Header.Large>

  <Router routes={screenRoutes} />
</SidebarLayout>
