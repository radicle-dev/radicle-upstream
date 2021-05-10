<script lang="typescript">
  import { fetchUser, user as store } from "../src/userProfile";

  import { Icon } from "../DesignSystem/Primitive";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header.svelte";
  import HorizontalMenu from "../DesignSystem/Component/HorizontalMenu.svelte";
  import Remote from "../DesignSystem/Component/Remote.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";

  import UserProfileHeader from "./UserProfile/UserProfileHeader.svelte";
  import ProjectsTab from "ui/screen/UserProfile/Projects.svelte";

  export let urn;
  export let activeTab: typeof SvelteComponent = ProjectsTab;

  const topbarMenuItems = [
    {
      icon: Icon.ChevronLeftRight,
      title: "Projects",
      tab: { component: ProjectsTab, props: { urn } },
      looseActiveStateMatching: true,
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
      <HorizontalMenu slot="left" items={topbarMenuItems} />
    </ActionBar>
    <svelte:component this={activeTab} />
  </Remote>
</SidebarLayout>
