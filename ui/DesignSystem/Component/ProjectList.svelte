<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../lib/path.js";
  import { projects, projectNameStore } from "../../src/project.ts";
  import * as remote from "../../src/remote.ts";
  import { session } from "../../src/session.ts";

  import { Flex, Text, Button } from "../Primitive";
  import ProjectCard from "./ProjectCard.svelte";
  import Placeholder from "./Placeholder.svelte";

  let entityId = null;

  // TODO(rudolfs): how do we make sure that this gets loaded before we render
  // the component?
  if ($session.status === remote.Status.Success) {
    entityId = $session.data.identity.id;
  }
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
</style>

{#if $projects.status === remote.Status.Success}
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
            projectId={project.id}
            {entityId}
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
{:else if $projects.status === remote.Status.Error}
  <Text>{`Error`}</Text>
{/if}
