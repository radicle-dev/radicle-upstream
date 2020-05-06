<script>
  import { createEventDispatcher } from "svelte";
  import ProjectCard from "./ProjectCard.svelte";

  export let projects = null;
  export let contextMenuItems = () => [];

  const dispatch = createEventDispatcher();
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
    border-bottom: 1px solid var(--color-foreground-level-3);
    cursor: pointer;
    padding: 22px 15px 26px 12px;
  }

  li:hover {
    background-color: var(--color-foreground-level-1);
  }

  li:last-child {
    border-bottom: 0;
  }
</style>

<ul>
  {#each projects as project}
    <li
      on:click={() => {
        dispatch('select', project);
      }}
      class="project-card">
      <ProjectCard
        title={project.metadata.name}
        projectId={project.id}
        contextMenuItems={contextMenuItems(project.id)}
        description={project.metadata.description}
        isRegistered={false}
        stats={project.stats} />
    </li>
  {/each}
</ul>
