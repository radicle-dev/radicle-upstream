<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { SvelteComponent } from "svelte";

  export let variant:
    | "primary"
    | "vanilla"
    | "transparent"
    | "primary-outline"
    | "outline"
    | "destructive"
    | "embedded" = "primary";

  export let disabled: boolean = false;
  export let transition: boolean = true;
  export let icon: typeof SvelteComponent | undefined = undefined;
  export let style: string | undefined = undefined;
  export let dataCy: string | undefined = undefined;
  export let title: string | undefined = undefined;

  $: buttonClass = [
    variant,
    icon && "icon",
    disabled && "disabled",
    transition && !disabled ? "button-transition" : "",
  ].join(" ");
</script>

<style>
  .button {
    white-space: nowrap;
    background-color: var(--color-foreground-level-2);
    border-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    cursor: pointer;
    display: flex;
    align-items: center;
    height: 2.5rem;
    outline-style: none;
    padding: 0 12px;
    user-select: none;
    line-height: 22px;
    gap: 0.5rem;
  }

  .button :global(svg) {
    fill: var(--color-foreground-level-6);
  }

  .button:hover {
    opacity: 0.75;
  }

  .primary :global(svg),
  .destructive :global(svg) {
    fill: var(--color-background);
  }

  .vanilla.disabled :global(svg) {
    fill: var(--color-foreground-level-5);
  }

  .vanilla:hover {
    border-color: var(--color-foreground-level-2);
  }

  .vanilla.disabled {
    background-color: var(--color-foreground-level-1);
    border-color: var(--color-foreground-level-1);
    color: var(--color-foreground-level-3);
    cursor: not-allowed;
  }

  .vanilla.disabled:hover {
    box-shadow: 0 0 0 0;
    border-color: var(--color-foreground-level-1);
    color: var(--color-foreground-level-3);
  }

  .vanilla.disabled :global(svg) {
    fill: var(--color-foreground-level-3);
  }

  .primary {
    background-color: var(--color-primary);
    border-color: var(--color-primary);
    color: var(--color-background);
  }

  .primary.disabled {
    opacity: 0.25;
    cursor: not-allowed;
  }

  .primary.disabled :global(svg) {
    fill: var(--color-background);
  }

  .primary-outline {
    background-color: transparent;
    border-color: var(--color-primary-level-2);
    color: var(--color-foreground);
  }
  .primary-outline:hover {
    color: var(--color-foreground);
    background-color: var(--color-primary);
    border: 1px solid var(--color-primary);
  }
  .primary-outline.disabled {
    opacity: 0.25;
    cursor: not-allowed;
  }
  .primary-outline.disabled :global(svg) {
    fill: var(--color-background);
  }

  .transparent {
    background-color: rgba(var(--color-background), 0);
    border-color: transparent;
    color: var(--color-foreground-level-6);
  }

  .transparent :global(svg) {
    fill: var(--color-foreground-level-5);
  }

  .transparent:hover {
    background-color: var(--color-foreground-level-2);
    border-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
  }

  .transparent.disabled {
    background-color: rgba(var(--color-background), 0);
    border-color: transparent;
    color: var(--color-foreground-level-3);
    cursor: not-allowed;
  }

  .transparent.disabled:hover {
    background-color: rgba(var(--color-background), 0);
    box-shadow: 0 0 0 0;
    border-color: transparent;
    color: var(--color-foreground-level-3);
    transform: scale(1);
  }

  .transparent.disabled :global(svg) {
    fill: var(--color-foreground-level-3);
  }

  .outline {
    background-color: rgba(var(--color-background), 0);
    border-color: var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
  }

  .outline :global(svg) {
    fill: var(--color-foreground-level-5);
  }

  .outline:hover {
    background-color: var(--color-foreground-level-1);
    border: 1px solid var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
  }

  .outline.disabled {
    background-color: rgba(var(--color-background), 0);
    border-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-3);
    cursor: not-allowed;
  }

  .outline.disabled:hover {
    background-color: rgba(var(--color-background), 0);
    box-shadow: 0 0 0 0;
    border-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-3);
    transform: scale(1);
  }

  .outline.disabled :global(svg) {
    fill: var(--color-foreground-level-3);
  }

  .destructive {
    background-color: var(--color-negative);
    border-color: var(--color-negative);
    color: var(--color-background);
  }

  .destructive:hover {
    color: var(--color-background);
  }

  .destructive.disabled {
    background-color: var(--color-negative-level-1);
    border-color: var(--color-negative-level-1);
    color: var(--color-background);
    cursor: not-allowed;
  }

  .destructive.disabled:hover {
    background-color: var(--color-negative-level-1);
    box-shadow: 0 0 0 0;
    border-color: var(--color-negative-level-1);
    color: var(--color-background);
    transform: scale(1);
  }

  .destructive.disabled :global(svg) {
    fill: var(--color-background);
  }

  .embedded {
    background: none;
    border: none;
    color: var(--color-foreground-level-6);
  }

  .embedded :global(svg) {
    fill: var(--color-foreground-level-5);
    margin: 0px;
  }

  .embedded:hover {
    color: var(--color-foreground-level-3);
  }

  .embedded:hover :global(svg) {
    fill: var(--color-foreground-level-3);
  }

  .embedded:active :global(svg) {
    fill: var(--color-foreground-level-4);
  }

  .embedded.disabled {
    fill: var(--color-foreground-level-3);
    cursor: not-allowed;
  }

  .embedded.disabled:hover,
  .embedded.disabled:active {
    box-shadow: 0 0 0 0;
    fill: var(--color-foreground-level-3);
  }

  .embedded.disabled :global(svg) {
    fill: var(--color-foreground-level-3);
  }
</style>

<button
  data-cy={dataCy}
  class="button typo-text-bold {buttonClass}"
  {title}
  {disabled}
  on:click|stopPropagation
  on:mouseenter
  on:mouseleave
  {style}>
  <svelte:component this={icon} />
  <slot />
</button>
