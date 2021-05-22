<script lang="typescript">
  import { isExperimental } from "../src/config";
  import * as modal from "../src/modal";
  import * as sess from "../src/session";
  import { settings } from "../src/session";

  import { Button, Icon } from "../DesignSystem/Primitive";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header.svelte";
  import TabBar from "../DesignSystem/Component/TabBar.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";
  import ProfileHeader from "./Profile/ProfileHeader.svelte";

  import ModalNewProject from "../Modal/NewProject.svelte";

  import FollowingTab from "ui/Screen/Profile/Following.svelte";
  import ProjectsTab from "ui/Screen//Profile/Projects.svelte";
  import FundingTab from "ui/Screen/Profile/Funding.svelte";

  type Tab = "projects" | "following" | "funding";
  export let activeTab: Tab = "projects";

  const tabs = (active: Tab) => {
    const tabs = [
      {
        title: "Projects",
        active: active === "projects",
        icon: Icon.ChevronLeftRight,
        onClick: () => {
          activeTab = "projects";
        },
      },
      {
        title: "Following",
        active: active === "following",
        icon: Icon.Network,
        onClick: () => {
          activeTab = "following";
        },
      },
    ];

    if (isExperimental && $settings.featureFlags.funding) {
      tabs.push({
        title: "Funding",
        active: active === "funding",
        icon: Icon.Wallet,
        onClick: () => {
          activeTab = "funding";
        },
      });
    }

    return tabs;
  };

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
    <TabBar slot="left" tabs={tabs(activeTab)} />
  </ActionBar>

  {#if activeTab === "projects"}
    <ProjectsTab />
  {:else if activeTab === "following"}
    <FollowingTab />
  {:else if activeTab === "funding"}
    <FundingTab />
  {/if}
</SidebarLayout>
