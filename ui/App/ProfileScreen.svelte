<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { unreachable } from "ui/src/unreachable";
  import * as Session from "ui/src/session";
  import * as modal from "ui/src/modal";
  import * as router from "ui/src/router";

  import { Button, Icon } from "ui/DesignSystem";

  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import Header from "ui/App/ScreenLayout/Header.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import FollowingTab from "./ProfileScreen/Following.svelte";
  import ProfileHeader from "./ProfileScreen/ProfileHeader.svelte";
  import ProjectsTab from "./ProfileScreen/Projects.svelte";

  import CreateProjectModal from "ui/App/CreateProjectModal.svelte";

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

  const session = Session.unsealed();
</script>

<ScreenLayout style="margin-top: 0;" dataCy="profile-screen">
  <Header>
    <ProfileHeader
      slot="left"
      urn={session.identity.urn}
      name={session.identity.metadata.handle}
      peerId={session.identity.peerId} />

    <Button
      slot="right"
      dataCy="new-project-button"
      variant="outline"
      icon={Icon.Plus}
      on:click={() => {
        modal.toggle(CreateProjectModal);
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
</ScreenLayout>
