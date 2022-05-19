<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let style: string | undefined = undefined;

  let wrapper: Element;
  let scrollHeight: number;
  let scrollY = 0;

  $: if (wrapper) {
    scrollHeight = wrapper.getBoundingClientRect().top;
  }
</script>

<style>
  .action-bar-wrapper {
    background-color: var(--color-background);
    position: sticky;
    top: 0;
    z-index: 1;
  }

  .action-bar {
    display: flex;
    height: var(--topbar-height);
    width: 100%;
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
    margin: 0 auto;
    align-items: center;
  }

  .elevation {
    box-shadow: var(--elevation-low);
  }
</style>

<svelte:window bind:scrollY />

<div
  bind:this={wrapper}
  class="action-bar-wrapper"
  class:elevation={scrollY > scrollHeight}>
  <div data-cy="action-bar" class="action-bar" {style}>
    <slot />
  </div>
</div>
