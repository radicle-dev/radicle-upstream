<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Cross from "design-system/icons/Cross.svelte";
  import Merge from "design-system/icons/Merge.svelte";
  import RevisionIcon from "design-system/icons/Revision.svelte";

  export let status: "open" | "closed" | "merged";

  $: iconColor = {
    open: "var(--color-primary)",
    closed: "var(--color-negative)",
    merged: "var(--color-positive)",
  }[status];
</script>

<style>
  .patch-icon {
    height: 24px;
    width: 24px;
  }
</style>

<div class="patch-icon">
  {#if status === "open"}
    <RevisionIcon style={`margin-right: 0.5rem; fill: ${iconColor};`} />
  {:else if status === "closed"}
    <Cross style={`margin-right: 0.5rem; fill: ${iconColor};`} />
  {:else if status === "merged"}
    <Merge style={`margin-right: 0.5rem; fill: ${iconColor};`} />
  {/if}
</div>
