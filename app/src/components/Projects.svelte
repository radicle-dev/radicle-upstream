<script>
  import { Text, Title, Numeric, Caption } from "../DesignSystem";
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { link } from "svelte-spa-router";

  const GET_PROJECTS = gql`
    query Query {
      projects {
        id
        description
        name
        imgUrl
      }
    }
  `;

  const client = getClient();
  const projects = query(client, { query: GET_PROJECTS });
</script>

{#await $projects}
  <Text.Regular>Loading projects...</Text.Regular>
{:then result}
  <ul>
    {#each result.data.projects as project}
      <li>
        <a href="/projects/{project.id}" use:link>
          {project.name} ({project.id}) - {project.description}
        </a>
      </li>
    {/each}
  </ul>
{:catch error}
  <p>ERROR: {error}</p>
{/await}
