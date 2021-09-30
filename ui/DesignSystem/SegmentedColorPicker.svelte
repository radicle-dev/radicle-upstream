<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { fly } from "svelte/transition";
  import { backOut } from "svelte/easing";
  import { createEventDispatcher } from "svelte";
  export let style: string | undefined = undefined;
  export let colorValue: string | undefined = undefined;
  const dispatch = createEventDispatcher();

  interface Option<T> {
    title: string;
    value: T;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type AnyOption = Option<any>;

  // Currently active option value.
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let active: any;
  // The available options.
  export let options: AnyOption[];

  const onClick = (option: AnyOption) => {
    dispatch("select", option.value);
    currentlyActive = option.value;
  };

  $: currentlyActive = active;
</script>

<style>
  .segmented-control {
    overflow: hidden;
    display: flex;
    width: fit-content;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    height: 2.5rem;
  }
  .segmented-control:hover button.active:not(:hover) {
    background: none;
  }
  .segmented-control button {
    cursor: pointer;
    max-height: 1.875rem;
    padding: 0rem 0.75rem;
    border-radius: 0.25rem;
    margin: 0.25rem;
    background-color: var(--color-background);
    color: var(--color-foreground-level-6);
  }
  .segmented-control button:focus {
    outline: none;
  }
  .segmented-control button.active {
    background-color: var(--color-foreground-level-2);
    color: var(--color-primary);
  }
  .segmented-control button:hover {
    background-color: var(--color-foreground-level-2);
  }
  .segmented-control button:active {
    background-color: var(--color-foreground-level-2);
  }

  input[type="color"] {
    -webkit-appearance: none;
    margin: 0.25rem;
    width: 1.875rem;
    height: 1.875rem;
  }
  input[type="color"]::-webkit-color-swatch-wrapper {
    padding: 0;
  }
  input[type="color"]::-webkit-color-swatch {
    border: none;
    border-radius: 0.25rem;
  }
</style>

<div class="segmented-control" {style}>
  {#each options as option}
    <button
      class="typo-semi-bold button-transition"
      class:active={option.value === currentlyActive}
      data-cy="segmented-control-option"
      value={option.value}
      on:click={() => onClick(option)}>
      {option.title}
    </button>
  {/each}
  {#if active === "custom"}
    <input
      in:fly={{ x: 30, duration: 100, easing: backOut }}
      out:fly={{ x: 30, duration: 100 }}
      {style}
      type="color"
      bind:value={colorValue} />
  {/if}
</div>
