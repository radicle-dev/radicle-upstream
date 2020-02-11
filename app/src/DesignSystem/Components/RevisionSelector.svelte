<script>
  import { gql } from "apollo-boost";
  import { getContext } from "svelte";
  import { getClient, query } from "svelte-apollo";

  import { revisionStore } from "../../stores/sourceBrowsing.js";
  import { Input } from "../Primitives";

  const ALL_REVISIONS = gql`
    query($projectId: ID!) {
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
  <Input.Dropdown
    dataCy="revision-selector"
    style="margin-bottom: 24px"
    items={[...result.data.tags, ...result.data.branches].sort()}
    bind:value={$revisionStore} />
{/await}
