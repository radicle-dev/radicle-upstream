<script lang="typescript">
  import * as modal from "../src/modal";
  import * as sess from "../src/session";
  import { unreachable } from "ui/src/unreachable";
  import * as router from "ui/src/router";

  import { Button, Icon } from "../DesignSystem/Primitive";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header.svelte";
  import TabBar from "ui/DesignSystem/Component/TabBar.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";
  import ProfileHeader from "./Profile/ProfileHeader.svelte";

  import ModalNewProject from "../Modal/NewProject.svelte";

  import FollowingTab from "ui/Screen/Profile/Following.svelte";
  import ProjectsTab from "ui/Screen//Profile/Projects.svelte";

  export let activeTab: router.ProfileTab;

  const tabs = (active: router.ProfileTab) => {
    return [
      {
        title: "Projects",
        active: active === "projects",
        icon: Icon.ChevronLeftRight,
        onClick: () => {
          router.push({ type: "profile", activeTab: "projects" });
        },
      },
      {
        title: "Following",
        active: active === "following",
        icon: Icon.Network,
        onClick: () => {
          router.push({ type: "profile", activeTab: "following" });
        },
      },
    ];
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
  {:else}
    {unreachable(activeTab)}
  {/if}
</SidebarLayout>
