<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Sidebar from "./Sidebar.svelte";

  export let dataCy: string | undefined = undefined;
</script>

<style>
  .container {
    position: relative;
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
    min-width: 60rem;
    height: 100%;
  }

  .content {
    padding-bottom: 1rem;
  }

  .header {
    height: 12.5rem;
    background-color: var(--color-foreground-level-1);
    display: flex;
  }

  .header-content {
    max-width: var(--content-max-width);
    margin: auto;
    flex-grow: 1;
    padding: 0 var(--content-padding);
  }
</style>

<div data-cy={dataCy}>
  <Sidebar />

  <div class="container">
    {#if $$slots.header}
      <div class="header" data-cy="header">
        <div class="header-content">
          <slot name="header" />
        </div>
      </div>
    {/if}

    <div class="content">
      <slot />
    </div>
  </div>
</div>
