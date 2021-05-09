<script lang="typescript">
  import type { SvelteComponent } from "svelte";

  import { isExperimental } from "../src/config";
  import * as modal from "../src/modal";
  import * as sess from "../src/session";
  import { settings } from "../src/session";

  import { Button, Icon } from "../DesignSystem/Primitive";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header.svelte";
  import HorizontalMenu from "../DesignSystem/Component/HorizontalMenu.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";
  import ProfileHeader from "./Profile/ProfileHeader.svelte";

  import ModalNewProject from "../Modal/NewProject.svelte";

  import FollowingTab from "./Profile/Following.svelte";
  import ProjectsTab from "./Profile/Projects.svelte";
  import FundingTab from "./Profile/Funding.svelte";

  export let activeTab: typeof SvelteComponent = ProjectsTab;

  const topbarMenuItems = [
    {
      icon: Icon.ChevronLeftRight,
      title: "Projects",
      tab: ProjectsTab,
    },
    {
      icon: Icon.Network,
      title: "Following",
      tab: FollowingTab,
    },
  ];

  if (isExperimental && $settings.featureFlags.funding) {
    topbarMenuItems.push({
      icon: Icon.Wallet,
      title: "Funding",
      tab: FundingTab,
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
    <HorizontalMenu
      slot="left"
      items={topbarMenuItems}
      on:select={event => {
        activeTab = event.detail.tab;
      }} />
  </ActionBar>

  <svelte:component this={activeTab} />
</SidebarLayout>
