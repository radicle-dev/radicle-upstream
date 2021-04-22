<script lang="typescript">
  import Router from "svelte-spa-router";

  import * as org from "../src/org";
  import * as path from "../src/path";
  import { store } from "../src/wallet";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header/Large.svelte";
  import HorizontalMenu from "../DesignSystem/Component/HorizontalMenu.svelte";
  import { SidebarLayout } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  export let params: { address: string };

  import Projects from "./Org/Projects.svelte";
  import Members from "./Org/Members.svelte";

  const screenRoutes = {
    "/org/:address/projects": Projects,
    "/org/:address/members": Members,
    "*": Projects,
  };

  $: topbarMenuItems = [
    {
      icon: Icon.ChevronLeftRight,
      title: "Projects",
      href: path.orgProjects(params.address),
    },
    {
      icon: Icon.User,
      title: "Members",
      href: path.orgMembers(params.address),
    },
  ];

  import ProjectsMenu from "./Org/ProjectsMenu.svelte";
  import MembersMenu from "./Org/MembersMenu.svelte";
  const menuRoutes = {
    "/org/:address/projects": ProjectsMenu,
    "/org/:address/members": MembersMenu,
    "*": ProjectsMenu,
  };

  let gnosisSafeAddress = null;

  (async () => {
    try {
      gnosisSafeAddress = await org.getSafeAddr(
        params.address,
        $store.provider
      );
    } catch (err) {
      return null;
    }
  })();
</script>

<SidebarLayout>
  <Header name={params.address} />

  <!-- TODO: This should go into the header.
  {#if gnosisSafeAddress}
    Gnosis safe address: {gnosisSafeAddress}
  {/if}
  -->

  <ActionBar>
    <div slot="left">
      <HorizontalMenu items={topbarMenuItems} />
    </div>
    <div slot="right">
      <Router routes={menuRoutes} />
    </div>
  </ActionBar>

  <Router routes={screenRoutes} />
</SidebarLayout>
