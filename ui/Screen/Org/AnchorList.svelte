<script lang="typescript">
  import type * as theGraphApi from "ui/src/theGraphApi";
  import { Icon } from "ui/DesignSystem/Primitive";
  import { List } from "ui/DesignSystem/Component";

  export let anchors: theGraphApi.ProjectAnchor;
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
  .container {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
  }
  .header {
    display: flex;
    margin: 1.5rem 3rem 0.5rem;
    width: 100%;
  }
</style>

{#if anchors.length !== 0}
  <div class="container">
    <div class="header">
      <p class="typo-text-bold">Still looking…&nbsp;</p>
      <p style="color: var(--color-foreground-level-6);">
        These anchored projects haven't been found in your network yet.
      </p>
    </div>
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
      <Icon.At style="margin-right: 0.5rem;" />
      {anchor.projectId.replace("rad:git:", "")}
      <Icon.AnchorSmall
        style="fill: var(--color-primary); margin-left: 0.5rem;" />
    </div>
  </div>
</List>
