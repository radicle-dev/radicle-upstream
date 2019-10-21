<script>
  import { gql } from 'apollo-boost'
  import { getClient, query } from 'svelte-apollo'
  import { link } from 'svelte-spa-router'

  const GET_PROJECTS = gql`
    query Query{
      projects {
        id
        description
        name
        imgUrl
      }
    }
  `

  const client = getClient()
  const projects = query(client, { query: GET_PROJECTS })
</script>

<style>
  li {
    margin-bottom: 12px;
  }
</style>

<h2>Projects</h2>
{#await $projects}
  <p>Loading projects...</p>
{:then result}
  <ul>
  {#each result.data.projects as project}
    <li>
      <a href='/projects/{project.id}' use:link>
        <strong>{project.name}</strong>({project.id}) - {project.description}
      </a>
    </li>
  {/each}
  </ul>
{:catch error}
  <p>ERROR: {error}</p>
{/await}
