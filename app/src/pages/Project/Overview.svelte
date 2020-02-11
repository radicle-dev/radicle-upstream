<script>
  import { getContext } from "svelte";

  import { Flex, Text } from "../../DesignSystem/Primitives";

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

<Flex align="left">
  <Text variant="bigTitle">Overview</Text>
</Flex>

{#await $project}
  <Text>Loading project...</Text>
{:then result}
  <Text>{result.data.project.metadata.name}</Text>
  <Text>{result.data.project.metadata.description}</Text>
{:catch error}
  <Text>ERROR: {error}</Text>
{/await}
