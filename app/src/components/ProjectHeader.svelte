<script>
  import { link } from "svelte-spa-router";
  import { Icon, Title } from "../DesignSystem";
  import * as path from "../path.js";
  import ProjectSidebar from "../components/ProjectSidebar.svelte";

  export let style = null;
  export let project = null;
</script>

<style>
  .topbar {
    display: flex;
    flex-direction: row;
    align-items: center;
    background-color: var(--color-white);
    width: 100%;
    height: 61px;
    left: var(--slim-sidebar-width);
    border-bottom: 1px solid var(--color-lightgray);
  }

  .topbar img {
    width: 32px;
    height: 32px;
    margin-right: 12px;
    margin-left: 16px;
  }

  .project-name {
    display: flex;
    align-items: center;
    height: 100%;
    border-right: 1px solid var(--color-lightgray);
  }
</style>

<div data-cy="topbar" class="topbar" {style}>
  <a class="project-name" href={path.projectOverview(project.id)} use:link>
    <img alt="Project Avatar" src={project.metadata.imgUrl} />
    <Title.Regular style="color: var(--color-purple); margin-right: 24px">
      {project.metadata.name}
    </Title.Regular>
  </a>
  <ProjectSidebar />
</div>
