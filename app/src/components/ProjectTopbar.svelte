<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";
  import { link } from "svelte-spa-router";
  import { Icon, Button, Avatar } from "../DesignSystem";
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

  .project-name {
    display: flex;
    align-items: center;
    height: 100%;
    border-right: 1px solid var(--color-lightgray);
    padding-left: 16px;
    padding-right: 24px;
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
    <Avatar
      style="color: var(--color-purple)"
      title={name}
      imageUrl={avatarUrl}
      variant="project" />
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
