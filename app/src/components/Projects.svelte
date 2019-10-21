<script>
  import { gql } from 'apollo-boost'
  import { getClient, query } from 'svelte-apollo'

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
    <li><strong>{project.name}</strong>({project.id}) - {project.description}</li>
  {/each}
  </ul>
{:catch error}
  <p>ERROR: {error}</p>
{/await}
