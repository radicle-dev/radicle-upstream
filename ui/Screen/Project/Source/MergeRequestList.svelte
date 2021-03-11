<script lang="typescript">
  import type { MergeRequest } from "../../../src/source";

  import { List } from "../../../DesignSystem/Component";
  import MergeRequestCard from "./MergeRequestCard.svelte";

  export let mergeRequests: MergeRequest[];
</script>

<style>
  .list-item {
    display: flex;
    width: 100%;
    justify-content: space-between;
    padding: 1.375rem 1.5rem;
    align-items: center;
    min-width: 0;
  }
</style>

<List
  dataCy="merge-request-list"
  items={mergeRequests}
  on:select
  let:item={mergeRequest}
  style="margin: 0 auto; overflow: visible;">
  <div class="list-item" data-cy={`project-list-entry-${mergeRequest}`}>
    <MergeRequestCard {mergeRequest} />
  </div>
</List>
