<script lang="typescript">
  import type * as theGraphApi from "ui/src/theGraphApi";
  import {
    AnchorMetadataModal,
    List,
    RadicleId,
  } from "ui/DesignSystem/Component";

  export let anchors: theGraphApi.ProjectAnchor[];
  export let orgAddress: string;
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
  .anchor-row {
    display: flex;
    white-space: nowrap;
    width: -webkit-fill-available;
    color: var(--color-foreground-level-6);
  }
  .reset-cursor {
    cursor: default;
  }
  .header {
    display: flex;
    margin: 1.5rem 3rem 0.5rem;
    width: 100%;
  }
</style>

{#if anchors.length !== 0}
  <div class="header">
    <p class="typo-text-bold">Still looking…&nbsp;</p>
    <p style="color: var(--color-foreground-level-6);">
      These anchored projects haven't been found in your network yet.
    </p>
  </div>
{/if}

<List
  dataCy="project-list"
  items={anchors}
  let:item={anchor}
  style="margin: 0 auto;">
  <div
    class="reset-cursor list-item"
    data-cy={`project-list-entry-${anchor.id}`}>
    <div class="typo-text anchor-row">
      <RadicleId urn={anchor.projectId} />
      <AnchorMetadataModal {anchor} {orgAddress} />
    </div>
  </div>
</List>
