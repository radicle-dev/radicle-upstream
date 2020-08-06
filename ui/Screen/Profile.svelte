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

  import ProjectsMenu from "./Profile/ProjectsMenu.svelte";

  const fragments = view.create(profile.fragments, profile.Fragment.Projects);

  const topbarMenuItems: view.MenuItem<profile.Fragment>[] = [
    {
      icon: Icon.Source,
      key: profile.Fragment.Projects,
      title: "Projects",
    },
    {
      icon: Icon.Fund,
      key: profile.Fragment.Wallet,
      title: "Wallet",
    },
  ];

  const dropdownMenuItems = [
    {
      title: "New project",
      dataCy: "new-project",
      icon: Icon.Plus,
      event: () => console.log("navigate to project creation"),
    },
  ];

  const session = getContext("session");
  if (session.permissions.registerHandle) {
    dropdownMenuItems.push({
      title: "Register handle",
      dataCy: "register-handle",
      icon: Icon.Register,
      event: () => console.log("navigate to user registration"),
    });
  }
</script>

<SidebarLayout style="margin-top: 0;" dataCy="profile-screen">
  <Header.Large
    variant="profile"
    entity={session.identity}
    on:registerHandle={() => console.log("navigate to user registration")}>
    <div slot="left">
      <HorizontalMenu items={topbarMenuItems} nav={fragments} />
    </div>
    <div slot="right" style="display: flex">
      <ProjectsMenu />
      <AdditionalActionsDropdown
        dataCy="profile-context-menu"
        style="margin: 0 16px"
        headerTitle={session.identity.shareableEntityIdentifier}
        menuItems={dropdownMenuItems} />
    </div>
  </Header.Large>

  <ViewRouter nav={fragments} />
</SidebarLayout>
