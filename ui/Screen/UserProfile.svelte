<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { fetchUser, user as store } from "../src/userProfile";

  import {
    ActionBar,
    Header,
    Icon,
    Remote,
    SidebarLayout,
    TabBar,
  } from "ui/DesignSystem";

  import UserProfileHeader from "./UserProfile/UserProfileHeader.svelte";
  import ProjectsTab from "ui/Screen/UserProfile/Projects.svelte";

  export let urn: string;

  const tabs = [
    {
      title: "Projects",
      active: true,
      icon: Icon.ChevronLeftRight,
      onClick: () => {},
    },
  ];

  fetchUser(urn);
</script>

<SidebarLayout>
  <Remote {store} let:data={identity}>
    <Header>
      <UserProfileHeader
        slot="left"
        name={identity.metadata.handle}
        peerId={identity.peerId}
        avatarFallback={identity.avatarFallback} />
    </Header>

    <ActionBar>
      <TabBar slot="left" {tabs} />
    </ActionBar>
    <ProjectsTab {urn} />
  </Remote>
</SidebarLayout>
