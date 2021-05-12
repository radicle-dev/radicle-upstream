<script lang="typescript">
  import { fetchUser, user as store } from "../src/userProfile";

  import { Icon } from "../DesignSystem/Primitive";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header.svelte";
  import HorizontalMenu from "../DesignSystem/Component/HorizontalMenu.svelte";
  import Remote from "../DesignSystem/Component/Remote.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";

  import UserProfileHeader from "./UserProfile/UserProfileHeader.svelte";
  import ProjectsTab from "ui/Screen/UserProfile/Projects.svelte";

  export let urn: string;
  export let activeTab: "projects";

  const activeTabToComponent = {
    projects: ProjectsTab,
  };

  $: activeTabComponent = activeTabToComponent[activeTab];

  const topbarMenuItems = [
    {
      icon: Icon.ChevronLeftRight,
      title: "Projects",
      tab: { type: "userProfile", activeTab: "projects", urn: urn },
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
      <HorizontalMenu
        slot="left"
        items={topbarMenuItems}
        activeTab={activeTabComponent} />
    </ActionBar>
    <svelte:component this={activeTabComponent} />
  </Remote>
</SidebarLayout>
