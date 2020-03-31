<script>
  import { push } from "svelte-spa-router";

  import { projects, projectNameStore } from "../../store/project.ts";
  import { createProject, projectOverview } from "../../lib/path.js";

  import { Text, Button } from "../Primitive";
  import ProjectCard from "./ProjectCard.svelte";
</script>

<style>
  ul {
    min-width: 500px;
  }

  li {
    display: flex;
    width: 100%;
    height: 96px;
    flex: 1;
    border-bottom: 1px solid var(--color-lightgray);
    cursor: pointer;
    padding: 22px 15px 26px 12px;
  }

  li:hover {
    background-color: var(--color-almostwhite);
  }

  li:last-child {
    border-bottom: 0;
  }

  .wrapper {
    margin-top: 156px;
    display: flex;
    justify-content: center;
  }

  .create-project {
    text-align: center;
    width: 240px;
  }
</style>

{#if $projects.length > 0}
  <ul>
    {#each $projects as project}
      <li
        on:click={() => {
          projectNameStore.set(project.metadata.name);
          push(projectOverview(project.id));
        }}
        class="project-card">
        <ProjectCard
          title={project.metadata.name}
          description={project.metadata.description}
          isRegistered={false} />
      </li>
    {/each}
  </ul>
{:else}
  <div class="wrapper">
    <div class="create-project">
      <Text
        variant="title"
        style="color: var(--color-darkgray); margin-bottom: 13px">
        You have no projects
      </Text>
      <Text style="color: var(--color-gray)">
        Create a new project and share it with friends to get started
      </Text>
      <Button
        style="margin: 23px auto"
        variant="primary"
        on:click={() => {
          push(createProject());
        }}>
        Create a new project
      </Button>
    </div>
  </div>
{/if}
