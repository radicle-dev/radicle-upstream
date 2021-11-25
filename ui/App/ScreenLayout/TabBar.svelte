<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts" context="module">
  export interface Tab {
    title: string;
    active: boolean;
    icon: typeof SvelteComponent;
    counter?: number;
    onClick: () => void;
  }
</script>

<script lang="ts">
  import type { SvelteComponent } from "svelte";

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
    line-height: 130%;
    color: var(--color-foreground-level-6);
  }

  .title.active {
    color: var(--color-primary);
  }

  .counter {
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    padding: 0.1875rem 0.5rem;
    border-radius: 0.75rem;
    margin-left: 0.5rem;
  }
</style>

<ul data-cy="tab-bar" class="tab-bar" {style}>
  {#each tabs as tab}
    <li
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

      {#if tab.counter}
        <span class="counter typo-mono-bold" data-cy="counter"
          >{tab.counter}</span>
      {/if}
    </li>
  {/each}
</ul>
