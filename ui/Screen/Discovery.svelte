<script>
  import { getFeed } from "../src/discovery.ts";

  import { SidebarLayout } from "../DesignSystem/Component";
  import { Icon, Input, Title } from "../DesignSystem/Primitive";

  import Project from "./Discovery/Project.svelte";

  const projectFeed = getFeed();
</script>

<style>
  .container {
    margin: 64px 100px;
  }

  .projects {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    column-gap: 32px;
    row-gap: 32px;
  }
</style>

<SidebarLayout>
  <div class="container">
    <Title variant="huge" style="margin-bottom: 28px;">Discover</Title>
    <Input.Text
      placeholder="Filter projects"
      showLeftItem
      style="margin-bottom: 28px; max-width: 350px;">
      <div slot="left">
        <Icon.Search />
      </div>
    </Input.Text>

    <div class="projects">
      {#each projectFeed as project}
        <Project {project} />
      {/each}
    </div>
  </div>
</SidebarLayout>
