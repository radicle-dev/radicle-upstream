<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts" context="module">
  import { type Tab } from "./TabBar";
  export { type Tab };
</script>

<script lang="ts">
  import Counter from "design-system/Counter.svelte";

  export let tabs: Tab[];
  export let style: string | undefined = undefined;
</script>

<style>
  .tab-bar {
    display: flex;
    align-items: center;
    gap: 2rem;
  }

  .tab {
    display: flex;
    align-items: center;
    line-height: 100%;
    cursor: pointer;
  }

  .icon {
    margin-right: 0.5rem;
    padding-top: 2px;
  }

  .title {
    color: var(--color-foreground-level-6);
  }

  .tab.active .title,
  .tab:hover .title {
    color: var(--color-primary);
  }

  .tab.active .icon :global(svg),
  .tab:hover .icon :global(svg) {
    fill: var(--color-primary);
  }
</style>

<div class="tab-bar" {style}>
  {#each tabs as tab}
    <div
      role="button"
      class="tab"
      class:active={tab.active}
      on:click={tab.onClick}>
      <div class="icon">
        <svelte:component
          this={tab.icon}
          style={tab.active ? "fill: var(--color-primary)" : ""} />
      </div>

      <div class="title typo-text-bold">
        {tab.title}
      </div>

      <Counter count={tab.counter} style="margin-left: 0.5rem;" />
    </div>
  {/each}
</div>
