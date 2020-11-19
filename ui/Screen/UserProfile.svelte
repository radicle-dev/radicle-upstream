<script lang="typescript">
  import Router from "svelte-spa-router";

  import * as path from "../src/path";
  import { fetchUser, user as store } from "../src/userProfile";

  import { Icon } from "../DesignSystem/Primitive";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header/Large.svelte";
  import HorizontalMenu from "../DesignSystem/Component/HorizontalMenu.svelte";
  import Remote from "../DesignSystem/Component/Remote.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";

  import Projects from "./UserProfile/Projects.svelte";
  import NotFound from "./NotFound.svelte";

  export let params: { urn: string };

  const screenRoutes = {
    "/user/:urn/projects": Projects,
    "*": NotFound,
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
    <Header
      name={identity.metadata.handle}
      peerId={identity.peerId}
      avatarFallback={identity.avatarFallback} />

    <ActionBar>
      <div slot="left">
        <HorizontalMenu items={topbarMenuItems} />
      </div>
    </ActionBar>

    <Router routes={screenRoutes} />
  </Remote>
</SidebarLayout>
