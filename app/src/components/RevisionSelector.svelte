<script>
  import { gql } from "apollo-boost";
  import { getContext } from "svelte";
  import { getClient, query } from "svelte-apollo";

  import { revision } from "../stores.js";
  import { Select } from "../DesignSystem";

  const ALL_REVISIONS = gql`
    query($projectId: IdInput!) {
      tags(id: $projectId)
      branches(id: $projectId)
    }
  `;

  const allRevisions = query(getClient(), {
    query: ALL_REVISIONS,
    variables: { projectId: getContext("projectId") }
  });
</script>

{#await $allRevisions then result}
  <Select
    dataCy="revision-selector"
    style="margin-bottom: 16px"
    items={[...result.data.tags, ...result.data.branches].sort()}
    bind:value={$revision} />
{/await}
