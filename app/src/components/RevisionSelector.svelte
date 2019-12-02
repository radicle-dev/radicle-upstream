<script>
  import { getContext } from "svelte";

  import ApolloClient from "apollo-boost";
  import { gql } from "apollo-boost";
  import { query } from "svelte-apollo";

  import { revision } from "../stores.js";

  import { Select } from "../DesignSystem";

  const client = new ApolloClient({
    uri: "http://127.0.0.1:4000"
  });

  const ALL_REVISIONS = gql`
    query($projectId: IdInput!) {
      tags(projectId: $projectId)
      branches(projectId: $projectId)
    }
  `;

  const allRevisions = query(client, {
    query: ALL_REVISIONS,
    variables: { projectId: getContext("projectId") }
  });
</script>

{#await $allRevisions then result}
  <Select
    style="margin-bottom: 16px"
    items={[...result.data.tags, ...result.data.branches]}
    bind:value={$revision} />
{/await}
