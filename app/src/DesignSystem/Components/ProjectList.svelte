<script>
  import { Text, Icon, Button } from "../Primitives";

  import ProjectCard from "./ProjectCard.svelte";

  import { projectNameStore } from "../../stores/project.js";
  import { createProject, projectOverview } from "../../path.js";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { link, push } from "svelte-spa-router";

  const GET_PROJECTS = gql`
    query Query {
      projects {
        id
        metadata {
          description
          name
          imgUrl
        }
      }
    }
  `;

  const client = getClient();
  const projects = query(client, { query: GET_PROJECTS });
  projects.refetch();
</script>

<style>
  li {
    display: flex;
    width: 100%;
    flex: 1;
    border-bottom: 1px solid var(--color-lightgray);
    cursor: pointer;
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

{#await $projects}
  <Text>Loading projects...</Text>
{:then result}
  {#if result.data.projects.length > 0}
    <ul>
      {#each result.data.projects as project}
        <li
          on:click={() => {
            projectNameStore.set(project.metadata.name);
            push(projectOverview(project.id));
          }}
          class="project-card">
          <ProjectCard
            title={project.metadata.name}
            description={project.metadata.description}
            isRegistered={false}
            imgUrl={project.metadata.imgUrl}
            state={Icon.Check} />
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
{:catch error}
  <p>ERROR: {error}</p>
{/await}
