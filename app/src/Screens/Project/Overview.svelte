<script>
  import { getContext } from "svelte";

  import { Flex, Text } from "../../DesignSystem/Primitives";
  import { SourceBrowser } from "../../DesignSystem/Components";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { registerProject } from "../../lib/path.js";

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

<Flex align="left">
  <Text variant="bigTitle">Overview</Text>
</Flex>

<div class="header">
  {#await $project then result}
    <Text>{result.data.project.metadata.name}</Text>
    <Text>{result.data.project.metadata.description}</Text>
  {/await}
</div>

<SourceBrowser />
