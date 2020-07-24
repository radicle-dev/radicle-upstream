<script>
  import { feed, fetch } from "../src/discovery.ts";

  import { Remote, SidebarLayout } from "../DesignSystem/Component";
  import { Icon, Input, Title } from "../DesignSystem/Primitive";

  import Project from "./Discovery/Project.svelte";

  fetch();
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 64px auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
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
      <Remote store={feed} let:data={projects}>
        {#each projects as project}
          <Project {project} />
        {/each}
      </Remote>
    </div>
  </div>
</SidebarLayout>
