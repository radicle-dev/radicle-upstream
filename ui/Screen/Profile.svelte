<script lang="typescript">
  import { getContext } from "svelte";
  import Router from "svelte-spa-router";

  import { isExperimental } from "../../native/ipc.js";
  import * as path from "../src/path";
  import type { UnsealedSession } from "../src/session";

  import {
    Header,
    HorizontalMenu,
    SidebarLayout,
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  import Following from "./Profile/Following.svelte";
  import Projects from "./Profile/Projects.svelte";
  import Funding from "./Profile/Funding.svelte";
  import NotFound from "./NotFound.svelte";

  const screenRoutes = {
    "/profile/following": Following,
    "/profile/projects": Projects,
    "/profile/funding": Funding,
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

  if (isExperimental()) {
    topbarMenuItems.push({
      icon: Icon.Wallet,
      title: "Funding",
      href: path.profileFunding(),
      looseActiveStateMatching: false,
    });
  }

  const session: UnsealedSession = getContext("session");
</script>

<SidebarLayout style="margin-top: 0;" dataCy="profile-screen">
  <Header.Large
    avatarFallback={session.identity.avatarFallback}
    name={session.identity.metadata.handle}
    peerId={session.identity.peerId}>
    <div slot="left">
      <HorizontalMenu items={topbarMenuItems} />
    </div>
    <div slot="top">
      <Router routes={menuRoutes} />
    </div>
  </Header.Large>

  <Router routes={screenRoutes} />
</SidebarLayout>
