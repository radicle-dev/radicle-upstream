<script>
  import { Text, Title, Icon, Numeric, Caption } from "../DesignSystem";
  import ProjectCard from "./ProjectCard.svelte";
  import { ProjectOverview } from "../routes.js";
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { link } from "svelte-spa-router";

  const GET_PROJECTS = gql`
    query Query {
      projects {
        id {
          name
          domain
        }
        description
        name
        imgUrl
      }
    }
  `;

  const client = getClient();
  const projects = query(client, { query: GET_PROJECTS });
</script>

<style>
  li {
    display: flex;
    width: 100%;
    flex: 1;
    border-bottom: 1px solid var(--color-lightgray);
  }

  li:hover {
    background-color: var(--color-almostwhite);
  }

  li:last-child {
    border-bottom: 0;
  }

  a {
    display: flex;
    width: 100%;
  }
</style>

{#await $projects}
  <Text.Regular>Loading projects...</Text.Regular>
{:then result}
  <ul>
    {#each result.data.projects as project}
      <li class="project-card">
        <a href={ProjectOverview.path(project.id)} use:link>
          <ProjectCard
            title={project.name}
            description={project.description}
            isRegistered={true}
            imgUrl={project.imgUrl}
            state={Icon.Check} />
        </a>
      </li>
    {/each}
  </ul>
{:catch error}
  <p>ERROR: {error}</p>
{/await}
