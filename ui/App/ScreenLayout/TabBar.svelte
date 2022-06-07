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
    align-items: center;
    display: flex;
    flex-direction: row;
  }

  .tab-bar :global(.tab:hover p) {
    color: var(--color-primary) !important;
  }

  .tab-bar :global(.tab:hover .icon svg) {
    fill: var(--color-primary);
  }

  .tab:first-child {
    margin-left: 0px;
  }

  .tab {
    margin-left: 24px;
    margin-right: 8px;
    align-items: center;
    line-height: 100%;
    display: flex;
    cursor: pointer;
  }

  .icon {
    margin-right: 0.5rem;
    align-items: center;
    padding-top: 2px;
  }

  .title {
    color: var(--color-foreground-level-6);
  }

  .title.active {
    color: var(--color-primary);
  }
</style>

<ul data-cy="tab-bar" class="tab-bar" {style}>
  {#each tabs as tab}
    <li
      role="button"
      aria-label={`${tab.title.toLowerCase()} tab`}
      class="tab"
      data-cy={`${tab.title.toLowerCase()}-tab`}
      on:click={tab.onClick}>
      <div class="icon">
        <svelte:component
          this={tab.icon}
          style={tab.active ? "fill: var(--color-primary)" : ""} />
      </div>

      <p class="title typo-text-bold" class:active={tab.active}>
        {tab.title}
      </p>

      <Counter count={tab.counter} style="margin-left: 0.5rem;" />
    </li>
  {/each}
</ul>
