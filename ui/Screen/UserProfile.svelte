<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as remote from "ui/src/remote";
  import * as userProfile from "ui/src/userProfile";

  import {
    ActionBar,
    Header,
    Icon,
    SidebarLayout,
    TabBar,
  } from "ui/DesignSystem";

  import UserProfileHeader from "./UserProfile/UserProfileHeader.svelte";
  import ProjectsTab from "./UserProfile/Projects.svelte";

  export let urn: string;

  const userProfileStore = userProfile.user;

  const tabs = [
    {
      title: "Projects",
      active: true,
      icon: Icon.ChevronLeftRight,
      onClick: () => {},
    },
  ];

  userProfile.fetchUser(urn);
</script>

<SidebarLayout dataCy="user-profile-screen">
  {#if $userProfileStore.status === remote.Status.Success}
    <Header>
      <UserProfileHeader
        slot="left"
        identityMetadata={$userProfileStore.data.metadata}
        deviceIds={$userProfileStore.data.peerIds}
        avatarFallback={$userProfileStore.data.avatarFallback} />
    </Header>

    <ActionBar>
      <TabBar slot="left" {tabs} />
    </ActionBar>
    <ProjectsTab {urn} />
  {/if}
</SidebarLayout>
