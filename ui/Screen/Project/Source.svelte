<script>
  import { getContext } from "svelte";

  import { Text, Title } from "../../DesignSystem/Primitive";
  import { SourceBrowser } from "../../DesignSystem/Component";

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
  const projectId = getContext("projectId");
  const project = query(client, {
    query: GET_PROJECT,
    variables: { projectId: projectId }
  });
</script>

<style>
  .header {
    margin-bottom: 32px;
  }
</style>

<div class="header">
  {#await $project then result}
    <Title variant="big">{result.data.project.metadata.name}</Title>
    <Text>{result.data.project.metadata.description}</Text>
  {/await}
</div>

<SourceBrowser />
