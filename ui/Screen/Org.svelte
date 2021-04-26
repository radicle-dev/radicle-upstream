<script lang="typescript">
  import Router from "svelte-spa-router";

  import * as org from "../src/org";
  import * as path from "../src/path";
  import { store } from "../src/wallet";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import AdditionalActionsDropdown from "../DesignSystem/Component/AdditionalActionsDropdown.svelte";
  import FollowToggle from "../DesignSystem/Component/FollowToggle.svelte";
  import Header from "../DesignSystem/Component/Header.svelte";
  import HorizontalMenu from "../DesignSystem/Component/HorizontalMenu.svelte";
  import { SidebarLayout } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  import OrgHeader from "./Org/OrgHeader.svelte";

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

  const additionalActionsDropdownItems = [
    {
      title: "Add something",
      icon: Icon.Plus,
      event: () => console.log("event(Add Something)"),
    },
  ];

  let gnosisSafeAddress = null;

  $: (async () => {
    try {
      gnosisSafeAddress = null;
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
  <Header>
    <OrgHeader slot="left" name={params.address} {gnosisSafeAddress} />
    <div slot="right" style="display: flex">
      <FollowToggle following disabled />
      <AdditionalActionsDropdown
        headerTitle={params.address}
        style="margin-left: 10px; border: 1px solid var(--color-foreground-level-3); border-radius: 4px;"
        menuItems={additionalActionsDropdownItems} />
    </div>
  </Header>

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
