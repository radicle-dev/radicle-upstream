<script>
  import { link } from "svelte-spa-router";
  import { Icon, Title } from "../DesignSystem";
  import * as path from "../path.js";
  import ProjectTopbarMenu from "../components/ProjectTopbarMenu.svelte";

  export let style = null;
  export let avatarUrl = null;
  export let id = null;
  export let name = null;
</script>

<style>
  .topbar {
    display: flex;
    flex-direction: row;
    align-items: center;
    background-color: var(--color-white);
    width: 100%;
    height: 61px;
    left: var(--sidebar-width);
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
  <a class="project-name" href={path.projectOverview(id)} use:link>
    <img alt="Project Avatar" src={avatarUrl} />
    <Title.Regular style="color: var(--color-purple); margin-right: 24px">
      {name}
    </Title.Regular>
  </a>
  <ProjectTopbarMenu />
</div>
