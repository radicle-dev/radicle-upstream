<script lang="typescript">
  import { fetchUser, user as store } from "../src/userProfile";

  import { Icon } from "../DesignSystem/Primitive";

  import ActionBar from "../DesignSystem/Component/ActionBar.svelte";
  import Header from "../DesignSystem/Component/Header.svelte";
  import TabBar from "ui/DesignSystem/Component/TabBar.svelte";
  import Remote from "../DesignSystem/Component/Remote.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";

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
