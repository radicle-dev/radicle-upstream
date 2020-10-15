<script lang="ts">
  import { getContext } from "svelte";
  import Router from "svelte-spa-router";

  import { isExperimental } from "../../native/ipc.js";
  import * as path from "../src/path";

  import {
    Header,
    HorizontalMenu,
    SidebarLayout,
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  import Projects from "./Profile/Projects.svelte";
  import Tracking from "./Profile/Tracking.svelte";
  import Funding from "./Profile/Funding.svelte";
  import NotFound from "./NotFound.svelte";

  const screenRoutes = {
    "/profile/projects": Projects,
    "/profile/funding": Funding,
    "/profile/tracking": Tracking,
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
      href: path.profileTracking(),
      looseActiveStateMatching: false,
    },
  ];

  if (isExperimental()) {
    topbarMenuItems.push({
      icon: Icon.Wallet,
      title: "Funding",
      href: path.profileFunding(),
      looseActiveStateMatching: false,
    });
  }

  const identity = getContext("session").identity;
</script>

<SidebarLayout style="margin-top: 0;" dataCy="profile-screen">
  <Header.Large
    name={identity.metadata.handle}
    urn={identity.shareableEntityIdentifier}
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
