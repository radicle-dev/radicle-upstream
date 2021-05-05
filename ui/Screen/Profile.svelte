<script lang="typescript">
  import Router from "svelte-spa-router";

  import { isExperimental } from "../src/config";
  import * as modal from "../src/modal";
  import * as path from "../src/path";
  import * as sess from "../src/session";
  import { settings } from "../src/session";

  import { Button, Icon } from "../DesignSystem/Primitive";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header.svelte";
  import HorizontalMenu from "../DesignSystem/Component/HorizontalMenu.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";
  import ProfileHeader from "./Profile/ProfileHeader.svelte";

  import ModalNewProject from "../Modal/NewProject.svelte";

  import Following from "./Profile/Following.svelte";
  import Projects from "./Profile/Projects.svelte";
  import Funding from "./Profile/Funding.svelte";

  const screenRoutes = {
    "/profile/following": Following,
    "/profile/projects": Projects,
    "/profile/funding": Funding,
  };

  const topbarMenuItems = [
    {
      icon: Icon.ChevronLeftRight,
      title: "Projects",
      href: path.profileProjects(),
    },
    {
      icon: Icon.Network,
      title: "Following",
      href: path.profileFollowing(),
    },
  ];

  if (isExperimental && $settings.featureFlags.funding) {
    topbarMenuItems.push({
      icon: Icon.Wallet,
      title: "Funding",
      href: path.profileFunding(),
    });
  }

  const session = sess.getUnsealedFromContext();
</script>

<SidebarLayout style="margin-top: 0;" dataCy="profile-screen">
  <Header>
    <ProfileHeader
      slot="left"
      avatarFallback={session.identity.avatarFallback}
      name={session.identity.metadata.handle}
      peerId={session.identity.peerId} />

    <Button
      slot="right"
      dataCy="new-project-button"
      variant="outline"
      icon={Icon.Plus}
      on:click={() => {
        modal.toggle(ModalNewProject);
      }}>
      New project
    </Button>
  </Header>

  <ActionBar>
    <HorizontalMenu slot="left" items={topbarMenuItems} />
  </ActionBar>

  <Router routes={screenRoutes} />
</SidebarLayout>
