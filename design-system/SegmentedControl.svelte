<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts" context="module">
  export interface SegmentedControlOption {
    title: string;
    value: string;
    icon?: typeof SvelteComponent;
  }
</script>

<script lang="ts">
  import { createEventDispatcher, SvelteComponent } from "svelte";

  export let style: string | undefined = undefined;

  export let active: string;
  export let options: SegmentedControlOption[];

  const dispatch = createEventDispatcher();

  const onClick = (option: SegmentedControlOption) => {
    dispatch("select", option.value);
    currentlyActive = option.value;
  };

  $: currentlyActive = active;
</script>

<style>
  .segmented-control {
    display: flex;
    width: fit-content;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    height: 2.5rem;
    background-color: var(--color-background);
    overflow: hidden;
  }

  button {
    cursor: pointer;
    max-height: 1.875rem;
    border-radius: 0.25rem;
    margin: 0.25rem;
    background-color: var(--color-background);
    color: var(--color-foreground-level-6);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  button:focus {
    outline: none;
  }

  button:hover,
  button:active {
    background-color: var(--color-foreground-level-2);
  }

  button.active {
    background-color: var(--color-foreground-level-2);
    color: var(--color-primary);
  }
</style>

<div class="segmented-control" {style}>
  {#each options as option}
    <button
      class="typo-semi-bold button-transition"
      class:active={option.value === currentlyActive}
      data-cy="segmented-control-option"
      value={option.value}
      style:padding={option.icon ? "0 0.75rem 0 0.5rem" : "0 0.75rem"}
      on:click={() => onClick(option)}>
      <svelte:component
        this={option.icon}
        style={option.value === currentlyActive
          ? "fill: var(--color-primary)"
          : "fill: var(--color-foreground-level-6)"} />
      {option.title}
    </button>
  {/each}

  <slot />
</div>
