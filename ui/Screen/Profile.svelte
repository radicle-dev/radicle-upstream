<script lang="ts">
  import { getContext } from "svelte";

  import * as profile from "../src/profile";
  import * as view from "../src/view";

  import {
    AdditionalActionsDropdown,
    Header,
    HorizontalMenu,
    SidebarLayout,
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  import ViewRouter from "../View/Router.svelte";

  import Projects from "./Profile/Projects.svelte";
  import Wallet from "./Profile/Wallet.svelte";

  import ProjectsMenu from "./Profile/ProjectsMenu.svelte";

  const fragments = view.create(profile.fragments, profile.Fragment.Projects);

  const menuRoutes = {
    "/profile/projects": ProjectsMenu,
  };

  const topbarMenuItems: profile.MenuItem[] = [
    {
      click: () => fragments.set(profile.Fragment.Projects),
      icon: Icon.Source,
      title: "Projects",
    },
    {
      click: () => fragments.set(profile.Fragment.Wallet),
      icon: Icon.Fund,
      title: "Wallet",
    },
  ];

  const dropdownMenuItems = [
    {
      title: "New project",
      dataCy: "new-project",
      icon: Icon.Plus,
      event: () => push(path.createProject()),
    },
  ];

  const session = getContext("session");
  if (session.permissions.registerHandle) {
    dropdownMenuItems.push({
      title: "Register handle",
      dataCy: "register-handle",
      icon: Icon.Register,
      event: () => push(path.registerUser()),
    });
  }
</script>

<SidebarLayout style="margin-top: 0;" dataCy="profile-screen">
  <Header.Large
    variant="profile"
    entity={session.identity}
    on:registerHandle={() => push(path.registerUser())}>
    <div slot="left">
      <HorizontalMenu items={topbarMenuItems} />
    </div>
    <div slot="right" style="display: flex">
      <Router routes={menuRoutes} />
      <AdditionalActionsDropdown
        dataCy="profile-context-menu"
        style="margin: 0 16px"
        headerTitle={session.identity.shareableEntityIdentifier}
        menuItems={dropdownMenuItems} />
    </div>
  </Header.Large>

  <ViewRouter views={fragments} />
</SidebarLayout>
