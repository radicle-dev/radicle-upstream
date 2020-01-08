<script>
  import { getContext } from "svelte";

  import { Header, Title } from "../../DesignSystem";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

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
  const project = query(client, {
    query: GET_PROJECT,
    variables: { projectId: getContext("projectId") }
  });
</script>

<Header>
  <div slot="left">
    <Title.Big>Overview</Title.Big>
  </div>
</Header>

{#await $project}
  <h1>Loading project...</h1>
{:then result}
  <h1>{result.data.project.metadata.name}</h1>
  <p>{result.data.project.metadata.description}</p>
{:catch error}
  <p>ERROR: {error}</p>
{/await}
