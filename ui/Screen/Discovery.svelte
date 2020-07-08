<script>
  import { getFeed } from "../src/discovery.ts";

  import { SidebarLayout } from "../DesignSystem/Component";
  import { Icon, Input, Title } from "../DesignSystem/Primitive";

  import Project from "./Discovery/Project.svelte";

  const projectFeed = getFeed();
</script>

<style>
  .container {
    margin: var(--content-padding) 96px;
  }

  .projects {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    grid-gap: 24px;
  }
</style>

<SidebarLayout>
  <div class="container">
    <Title variant="huge" style="margin-bottom: 32px;">Discover</Title>

    <div class="projects">
      <Input.Text
        placeholder="Filter projects"
        showLeftItem
        style="margin-bottom: 8px;">
        <div slot="left" style="display: flex;">
          <Icon.Search />
        </div>
      </Input.Text>

      <div />

      {#each projectFeed as project}
        <Project {project} />
      {/each}
    </div>
  </div>
</SidebarLayout>
