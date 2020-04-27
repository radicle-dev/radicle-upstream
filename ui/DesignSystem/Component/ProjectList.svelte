<script>
  import { createEventDispatcher } from "svelte";

  import ProjectCard from "./ProjectCard.svelte";
  import * as remote from "../../src/remote.ts";
  import { session } from "../../src/session.ts";

  // TODO(rudolfs): how do we make sure that this gets loaded before we render
  // the component?
  let registrarId = null;

  if ($session.status === remote.Status.Success) {
    registrarId = $session.data.identity.id;
  }

  export let projects = null;

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
        {registrarId}
        description={project.metadata.description}
        isRegistered={false}
        stats={project.stats} />
    </li>
  {/each}
</ul>
