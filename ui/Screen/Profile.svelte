<script lang="typescript">
  import { isExperimental } from "../src/config";
  import * as modal from "../src/modal";
  import * as sess from "../src/session";
  import { settings } from "../src/session";
  import type { HorizontalItem } from "ui/src/menu";

  import { Button, Icon } from "../DesignSystem/Primitive";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header.svelte";
  import HorizontalMenu from "../DesignSystem/Component/HorizontalMenu.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";
  import ProfileHeader from "./Profile/ProfileHeader.svelte";

  import ModalNewProject from "../Modal/NewProject.svelte";

  import FollowingTab from "ui/Screen/Profile/Following.svelte";
  import ProjectsTab from "ui/Screen//Profile/Projects.svelte";
  import FundingTab from "ui/Screen/Profile/Funding.svelte";

  export let activeTab: "projects" | "following" = "projects";

  const topbarMenuItems: HorizontalItem[] = [
    {
      icon: Icon.ChevronLeftRight,
      title: "Projects",
      tab: "projects",
    },
    {
      icon: Icon.Network,
      title: "Following",
      tab: "following",
    },
  ];

  if (isExperimental && $settings.featureFlags.funding) {
    topbarMenuItems.push({
      icon: Icon.Wallet,
      title: "Funding",
      tab: "funding",
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
      {activeTab}
      on:select={event => {
        activeTab = event.detail.tab;
      }} />
  </ActionBar>

  {#if activeTab === "projects"}
    <ProjectsTab />
  {:else if activeTab === "following"}
    <FollowingTab />
  {:else if activeTab === "funding"}
    <FundingTab />
  {/if}
</SidebarLayout>
