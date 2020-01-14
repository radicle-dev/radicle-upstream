<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";
  import { link } from "svelte-spa-router";
  import { Icon, Title, Button } from "../DesignSystem";
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
    justify-content: flex-start;
    background-color: var(--color-white);
    width: calc(100vw - var(--sidebar-width));
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

  .left {
    display: flex;
    flex: 1;
    justify-content: flex-end;
    margin-right: 16px;
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

  <div class="left">
    <Button
      variant="secondary"
      size="small"
      on:click={() => {
        push(path.registerProject(id));
      }}>
      Register
    </Button>
  </div>
</div>
