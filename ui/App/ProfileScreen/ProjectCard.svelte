<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as project from "ui/src/project";

  import Badge from "design-system/Badge.svelte";
  import ProjectAnchorHovercard from "ui/App/SharedComponents/ProjectAnchorHovercard.svelte";

  export let title: string;

  export let description: string | undefined = undefined;

  export let showDelegateBadge: boolean = false;
  export let anchor: project.Anchor | undefined;
</script>

<style>
  .project-card {
    display: flex;
    max-height: 3.2rem;
    flex-direction: column;
    justify-content: center;
    min-width: 0;
    margin-right: 1.5rem;
  }

  .title-row {
    display: flex;
    white-space: nowrap;
    width: -webkit-fill-available;
  }

  .title-row p {
    color: var(--color-foreground-level-6);
  }

  .desc {
    color: var(--color-foreground-level-6);
    margin-top: 0.125rem;
    width: -webkit-fill-available;
  }
</style>

<div class="project-card">
  <div class="title-row">
    <p class="typo-text-bold typo-overflow-ellipsis" {title}>{title}</p>
    {#if showDelegateBadge}
      <Badge style="margin-left: 0.5rem" text="delegate" />
    {/if}
    {#if anchor}
      <ProjectAnchorHovercard {anchor} replicated={true} />
    {/if}
  </div>
  {#if description}
    <p class="desc typo-overflow-ellipsis" title={description}>{description}</p>
  {/if}
</div>
