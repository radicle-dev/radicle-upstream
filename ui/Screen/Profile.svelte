<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as modal from "../src/modal";
  import * as Session from "ui/src/session";
  import { unreachable } from "ui/src/unreachable";
  import * as router from "ui/src/router";

  import {
    ActionBar,
    Button,
    Header,
    Icon,
    SidebarLayout,
    TabBar,
  } from "ui/DesignSystem";

  import FollowingTab from "./Profile/Following.svelte";
  import ProfileHeader from "./Profile/ProfileHeader.svelte";
  import ProjectsTab from "./Profile/Projects.svelte";

  import CreateProjectModal from "ui/Screen/Project/CreateProjectModal.svelte";

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
</SidebarLayout>
