<script>
  import { link } from "svelte-spa-router";
  import Layout from "../../components/Layout.svelte";
  import ProjectSidebar from "../../components/ProjectSidebar.svelte";

  export let params = {};

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  const GET_PROJECT = gql`
    query Query($id: ProjectId!) {
      project(id: $id) {
        name
        description
        imgUrl
        members {
          keyName
          avatarUrl
        }
      }
    }
  `;

  const client = getClient();
  const project = query(client, {
    query: GET_PROJECT,
    variables: { id: params.id }
  });
</script>

<Layout>
  <div slot="double">
    <ProjectSidebar projectId={params.id} />
  </div>

  {#await $project}
    <h1>Loading project...</h1>
  {:then result}
    <h1>{result.data.project.name}</h1>
    <p>{result.data.project.description}</p>
  {:catch error}
    <p>ERROR: {error}</p>
  {/await}
</Layout>
