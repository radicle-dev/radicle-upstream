<script lang="ts">
  import Router from "svelte-spa-router";

  import * as path from "../src/path";

  import {
    Header,
    HorizontalMenu,
    Remote,
    SidebarLayout,
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";
  import { fetch, identity as store } from "../src/identity";

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

  $: fetch({ urn: params.urn });
</script>

<SidebarLayout>
  <Remote {store} let:data={identity}>
    <Header.Large
      name={identity.metadata.handle}
      urn={identity.shareableEntityIdentifier}
      avatarFallback={identity.avatarFallback}>
      <div slot="left">
        <HorizontalMenu items={topbarMenuItems} />
      </div>
    </Header.Large>

    <Router routes={screenRoutes} />
  </Remote>
</SidebarLayout>
