<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as remote from "ui/src/remote";
  import * as userProfile from "ui/src/userProfile";

  import { Icon } from "ui/DesignSystem";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import Header from "ui/App/ScreenLayout/Header.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import UserProfileHeader from "./UserProfileScreen/UserProfileHeader.svelte";
  import ProjectsTab from "./UserProfileScreen/Projects.svelte";

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

<ScreenLayout dataCy="user-profile-screen">
  {#if $userProfileStore.status === remote.Status.Success}
    <Header>
      <UserProfileHeader
        slot="left"
        identityMetadata={$userProfileStore.data.metadata}
        deviceIds={$userProfileStore.data.peerIds}
        {urn} />
    </Header>

    <ActionBar>
      <TabBar slot="left" {tabs} />
    </ActionBar>
    <ProjectsTab {urn} />
  {/if}
</ScreenLayout>
