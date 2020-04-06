<script>
  import { push } from "svelte-spa-router";

  import { projects } from "../../lib/project.ts";
  import { currentProjectName } from "../../store/project.ts";
  import { createProject, projectSource } from "../../lib/path.js";

  import { Text, Button } from "../Primitive";
  import ProjectCard from "./ProjectCard.svelte";

  console.log($projects.status);
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

  .wrapper {
    margin-top: 156px;
    display: flex;
    justify-content: center;
  }

  .create-project {
    text-align: center;
    width: 240px;
  }

  .loading {
    height: 1000px;
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
  }

  .loading-text {
    animation: loadingText 1.2s infinite, spin 2000ms infinite linear;
    font-size: 10rem;
    font-family: "Comic Sans MS";
    z-index: 1000;
    position: absolute;
    top: 50%;
    left: 20%;
  }

  .loading-gif {
    z-index: 10;
    position: absolute;
    top: 30%;
    left: 30%;
  }

  @keyframes loadingText {
    0% {
      color: var(--color-primary);
    }
    20% {
      color: var(--color-secondary);
    }
    40% {
      color: var(--color-tertiary);
    }
    60% {
      color: transparent;
    }
    99% {
      color: var(--color-primary);
    }
    100% {
      color: var(--color-primary);
    }
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>

<!-- 
  SuccessEmpty state could work like this:
  {#if status === 'LOADING'}
    <Loading />
  {#else if status === 'SUCCESS'}
    render with data
    <List />
  {#else if status === 'Failure'}
    error state
  {/if} 
-->

{#if $projects.status === 'LOADING'}
  <div class="loading">
    <p class="loading-text">LOADING</p>
    <iframe
      title="loading"
      src="https://giphy.com/embed/13d42CVe9KZxQc"
      width="480"
      height="360"
      frameBorder="0"
      class="loading-gif"
      allowFullScreen />
  </div>
{:else if $projects.status === 'SUCCESS'}
  {#if $projects.data.projects.length > 0}
    <ul>
      {#each $projects.data.projects as project}
        <li
          on:click={() => {
            currentProjectName.update(project.name);
            push(projectSource(project.id));
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
    <!-- SuccessEmpty state -->
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
{:else if $projects.status === 'FAILURE'}
  <Text>{`Error`}</Text>
{/if}
