<script>
  import { getContext } from "svelte";

  import { Title } from "../../DesignSystem";
  import RowLayout from "../../layouts/helpers/Row.svelte";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { registerProject } from "../../path.js";

  const GET_PROJECT = gql`
    query Query($id: ID!) {
      project(id: $projectId) {
        metadata {
          name
          description
          imgUrl
        }
      }
    }
  `;

  const client = getClient();
  const projectId = getContext("projectId");
  const project = query(client, {
    query: GET_PROJECT,
    variables: { projectId: projectId }
  });
</script>

<RowLayout>
  <div slot="left">
    <Title size="big">Overview</Title>
  </div>
</RowLayout>

{#await $project}
  <h1>Loading project...</h1>
{:then result}
  <h1>{result.data.project.metadata.name}</h1>
  <p>{result.data.project.metadata.description}</p>
{:catch error}
  <p>ERROR: {error}</p>
{/await}
