<script lang="typescript">
  import Router from "svelte-spa-router";

  import * as path from "../src/path";
  import { fetchUser, user as store } from "../src/userProfile";

  import { Icon } from "../DesignSystem/Primitive";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header.svelte";
  import HorizontalMenu from "../DesignSystem/Component/HorizontalMenu.svelte";
  import Remote from "../DesignSystem/Component/Remote.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";

  import UserProfileHeader from "./UserProfile/UserProfileHeader.svelte";

  import Projects from "./UserProfile/Projects.svelte";

  export let params: { urn: string };

  const screenRoutes = {
    "/user/:urn/projects": Projects,
  };
  const topbarMenuItems = [
    {
      icon: Icon.ChevronLeftRight,
      title: "Projects",
      href: path.userProfileProjects(params.urn),
      looseActiveStateMatching: true,
    },
  ];

  fetchUser(params.urn);
</script>

<SidebarLayout>
  <Remote {store} let:data={identity}>
    <Header>
      <UserProfileHeader
        slot="left"
        name={identity.metadata.handle}
        peerId={identity.peerId}
        avatarFallback={identity.avatarFallback} />
    </Header>

    <ActionBar>
      <HorizontalMenu slot="left" items={topbarMenuItems} />
    </ActionBar>

    <Router routes={screenRoutes} />
  </Remote>
</SidebarLayout>
