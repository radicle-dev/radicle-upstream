<script lang="ts">
  import { getContext } from "svelte";
  import Router from "svelte-spa-router";

  import * as path from "../src/path";

  import {
    Header,
    HorizontalMenu,
    SidebarLayout,
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  import Following from "./Profile/Following.svelte";
  import Projects from "./Profile/Projects.svelte";
  import NotFound from "./NotFound.svelte";

  const screenRoutes = {
    "/profile/following": Following,
    "/profile/projects": Projects,
    "*": NotFound,
  };

  import ProjectsMenu from "./Profile/ProjectsMenu.svelte";

  const menuRoutes = {
    "/profile/projects": ProjectsMenu,
    "/profile/tracking": ProjectsMenu,
    "*": ProjectsMenu,
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
      href: path.profileFollowing(),
      looseActiveStateMatching: false,
    },
  ];

  const identity = getContext("session").identity;
</script>

<SidebarLayout style="margin-top: 0;" dataCy="profile-screen">
  <Header.Large
    name={identity.metadata.handle}
    urn={identity.urn}
    avatarFallback={identity.avatarFallback}>
    <div slot="left">
      <HorizontalMenu items={topbarMenuItems} />
    </div>
    <div slot="top">
      <Router routes={menuRoutes} />
    </div>
  </Header.Large>

  <Router routes={screenRoutes} />
</SidebarLayout>
