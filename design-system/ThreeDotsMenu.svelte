<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts" context="module">
  export interface MenuItem {
    title: string;
    icon: typeof SvelteComponent;
    event: () => void;
    tooltip?: string;
    dataCy?: string;
    disabled?: boolean;
  }
</script>

<script lang="ts">
  import type { SvelteComponent } from "svelte";
  import { fade } from "svelte/transition";

  import EllipsisIcon from "./icons/Ellipsis.svelte";
  import Overlay from "./Overlay.svelte";
  import Tooltip from "./Tooltip.svelte";

  export let menuItems: MenuItem[];
  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;

  let expanded = false;
</script>

<style>
  .container {
    position: relative;
    height: 2.5rem;
    width: 2.5rem;
  }

  button {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 0.5rem;
    cursor: pointer;
    outline-style: none;
    border: 1px solid var(--color-foreground-level-3);
  }

  button :global(svg) {
    fill: var(--color-foreground-level-6);
  }

  button:hover {
    background-color: var(--color-foreground-level-2);
  }

  .modal {
    background-color: var(--color-background);
    border-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-3);
    box-shadow: var(--elevation-medium);
    cursor: pointer;
    margin-top: 1rem;
    position: absolute;
    right: 0;
    top: 100%;
    user-select: none;
    width: 15rem;
    z-index: 10;
    /* Round corners of child elements. */
    overflow: hidden;
  }

  .item {
    cursor: pointer;
    display: flex;
    padding: 0.5rem 0.75rem;
    color: var(--color-foreground-level-6);
  }

  .item:hover {
    background-color: var(--color-foreground-level-1);
  }

  .item.disabled {
    color: var(--color-foreground-level-4);
    cursor: not-allowed;
  }

  .item.disabled :global(svg) {
    fill: var(--color-foreground-level-4);
  }
</style>

<Overlay
  {expanded}
  on:hide={() => {
    expanded = false;
  }}>
  <div data-cy={dataCy} class="container" {style}>
    <button
      class="button-transition"
      on:click|stopPropagation={() => {
        expanded = !expanded;
      }}>
      <EllipsisIcon />
    </button>
    {#if expanded && menuItems.length > 0}
      <div
        out:fade|local={{ duration: 100 }}
        class="modal"
        data-cy="dropdown-menu">
        {#each menuItems as item}
          <Tooltip value={item.tooltip} position="left">
            <div
              data-cy={item.dataCy}
              class="item"
              class:disabled={item.disabled}
              on:click={!item.disabled
                ? () => {
                    expanded = false;
                    item.event();
                  }
                : undefined}>
              <svelte:component
                this={item.icon}
                style="margin-right: 0.75rem;" />
              <div>{item.title}</div>
            </div>
          </Tooltip>
        {/each}
      </div>
    {/if}
  </div>
</Overlay>
