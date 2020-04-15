<script>
  import { push } from "svelte-spa-router";

  import { projects, projectNameStore } from "../../src/project.ts";
  import * as path from "../../lib/path.js";

  import { Flex, Text, Button } from "../Primitive";
  import ProjectCard from "./ProjectCard.svelte";
  import Placeholder from "./Placeholder.svelte";
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
    width: 480px;
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
  {#if $projects.data.length > 0}
    <ul>
      {#each $projects.data as project}
        <li
          on:click={() => {
            projectNameStore.set(project.metadata.name);
            push(path.projectSource(project.id));
          }}
          class="project-card">
          <ProjectCard
            title={project.metadata.name}
            description={project.metadata.description}
            isRegistered={false}
            stats={project.stats} />
        </li>
      {/each}
    </ul>
  {:else}
    <div class="wrapper">
      <div class="create-project">
        <Placeholder style="width: 420px; height: 217px;" />
        <Flex style="margin-top: 27px;">
          <div slot="left" style="align-items: center; justify-content: center">
            <Text
              style="margin-bottom: 24px; text-align: left; color:
              var(--color-foreground-level-6);">
              Create a new project because that's why you're here.
            </Text>
            <Button
              variant="vanilla"
              on:click={() => {
                push(path.createProject());
              }}>
              Start a new project
            </Button>
          </div>
          <div
            slot="right"
            style="margin-left: 24px; display: flex; flex-direction: column;
            align-items: center; justify-content: center">
            <Text
              style="margin-bottom: 24px; text-align: left; color:
              var(--color-foreground-level-6);">
              Register so your friends can find you!
            </Text>
            <Button
              variant="vanilla"
              on:click={() => {
                push(path.registerUser());
              }}>
              Register handle
            </Button>
          </div>
        </Flex>
      </div>
    </div>
  {/if}
{:else if $projects.status === 'ERROR'}
  <Text>{`Error`}</Text>
{/if}
