<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { onDestroy } from "svelte";
  import LoadingAnimation from "design-system/LoadingAnimation.svelte";

  export let style: string | undefined = undefined;

  let show: boolean = false;

  const timeout = window.setTimeout(() => {
    show = true;
  }, 100);

  onDestroy(() => {
    window.clearTimeout(timeout);
  });
</script>

<style>
  .loading-container {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    gap: 1.5rem;
  }
</style>

{#if show}
  <div class="loading-container" {style}>
    <LoadingAnimation />
    <slot />
  </div>
{/if}
