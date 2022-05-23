<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";

  export let name: string;
  export let urn: string;
  export let description: string | null = null;
  export let onClick: (() => void) | undefined = undefined;
</script>

<style>
  .metadata {
    display: flex;
    flex-direction: column;
    align-self: center;
    width: -webkit-fill-available;
    min-width: 0;
  }

  .description {
    margin-top: 1rem;
    margin-bottom: 0.5rem;
    color: var(--color-foreground-level-6);
  }

  .clickable {
    cursor: pointer;
  }
</style>

<div class="metadata">
  <h1
    data-cy="entity-name"
    class="typo-overflow-ellipsis"
    title={name}
    class:clickable={onClick !== undefined}
    on:click={() => (onClick ? onClick() : {})}>
    {name}
  </h1>
  <CopyableIdentifier
    value={urn}
    kind="projectId"
    style="margin-top: 0.5rem;" />
  {#if description}
    <p class="description typo-overflow-ellipsis" title={description}>
      {description}
    </p>
  {/if}
</div>
