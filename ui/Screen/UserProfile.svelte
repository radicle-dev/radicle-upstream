<script lang="typescript">
  import { fetchUser, user as store } from "../src/userProfile";

  import { Icon } from "ui/DesignSystem";

  import ActionBar from "ui/DesignSystem/ActionBar.svelte";
  import Header from "ui/DesignSystem/Header.svelte";
  import TabBar from "ui/DesignSystem/TabBar.svelte";
  import Remote from "ui/DesignSystem/Remote.svelte";
  import SidebarLayout from "ui/DesignSystem/SidebarLayout.svelte";

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
