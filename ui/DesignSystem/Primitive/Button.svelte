<script lang="ts">
  import type { SvelteComponent } from "svelte";

  import type { ButtonVariant as Variant } from "../../src/style";

  export let variant: Variant = "primary";

  export let disabled: boolean = false;
  export let icon: typeof SvelteComponent | undefined = undefined;
  export let style = "";
  export let dataCy = "";
  export let title = "";

  $: buttonClass = [variant, icon && "icon", disabled && "disabled"].join(" ");
</script>

<style>
  .button {
    white-space: nowrap;
    background-color: var(--color-foreground-level-3);
    border-radius: 4px;
    border: 1px solid var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
    cursor: pointer;
    display: flex;
    align-items: center;
    min-height: 40px;
    outline-style: none;
    padding: 0 12px;
    user-select: none;
    line-height: 22px;
  }

  .button :global(svg) {
    fill: var(--color-foreground-level-6);
  }

  .primary :global(svg),
  .secondary :global(svg),
  .destructive :global(svg) {
    fill: var(--color-background);
  }

  .vanilla.disabled :global(svg) {
    fill: var(--color-foreground-level-5);
  }

  .vanilla:hover {
    background-color: var(--color-foreground-level-2);
    border-color: var(--color-foreground-level-2);
  }

  .vanilla:active {
    color: var(--color-foreground-level-5);
  }

  .vanilla:active :global(svg) {
    fill: var(--color-foreground-level-5);
  }

  .vanilla.disabled {
    background-color: var(--color-foreground-level-1);
    border-color: var(--color-foreground-level-1);
    color: var(--color-foreground-level-3);
    cursor: not-allowed;
  }

  .vanilla.disabled:hover {
    background-color: var(--color-foreground-level-1);
    box-shadow: 0 0 0 0;
    border-color: var(--color-foreground-level-1);
    color: var(--color-foreground-level-3);
  }

  .vanilla.disabled:active {
    background-color: var(--color-foreground-level-1);
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

  .primary:hover {
    background-color: var(--color-primary-level-2);
    border-color: var(--color-primary-level-2);
    color: var(--color-background);
  }

  .primary:active {
    background-color: var(--color-primary-level-2);
    border-color: var(--color-primary-level-2);
    color: var(--color-primary-level-1);
  }

  .primary:active :global(svg) {
    fill: var(--color-primary-level-1);
  }

  .primary.disabled {
    background-color: var(--color-primary-level-1);
    border-color: var(--color-primary-level-1);
    color: var(--color-background);
    cursor: not-allowed;
  }

  .primary.disabled:hover {
    background-color: var(--color-primary-level-1);
    box-shadow: 0 0 0 0;
    border-color: var(--color-primary-level-1);
    color: var(--color-background);
  }

  .primary.disabled :global(svg) {
    fill: var(--color-background);
  }

  .primary.disabled:active {
    background-color: var(--color-primary-level-1);
    border-color: var(--color-primary-level-1);
    color: var(--color-background);
  }

  .secondary {
    background-color: var(--color-secondary);
    border-color: var(--color-secondary);
    color: var(--color-background);
  }

  .secondary:hover {
    background-color: var(--color-secondary-level-2);
    border-color: var(--color-secondary-level-2);
    color: var(--color-background);
  }

  .secondary:active {
    background-color: var(--color-secondary-level-2);
    border-color: var(--color-secondary-level-2);
    color: var(--color-secondary-level-1);
  }

  .secondary:active :global(svg) {
    fill: var(--color-secondary-level-1);
  }

  .secondary.disabled {
    background-color: var(--color-secondary-level-1);
    border-color: var(--color-secondary-level-1);
    color: var(--color-background);
    cursor: not-allowed;
  }

  .secondary.disabled:hover {
    background-color: var(--color-secondary-level-1);
    box-shadow: 0 0 0 0;
    border-color: var(--color-secondary-level-1);
    color: var(--color-background);
  }

  .secondary.disabled:active {
    background-color: var(--color-secondary-level-1);
    border-color: var(--color-secondary-level-1);
    color: var(--color-background);
  }

  .secondary.disabled :global(svg) {
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

  .transparent:active {
    background-color: var(--color-foreground-level-2);
    border-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-5);
  }

  .transparent:active :global(svg) {
    fill: var(--color-foreground-level-5);
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
  }

  .transparent.disabled:active {
    background-color: rgba(var(--color-background), 0);
    border-color: transparent;
    color: var(--color-foreground-level-3);
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

  .outline:active {
    background-color: var(--color-foreground-level-1);
    border-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-5);
  }

  .outline:active :global(svg) {
    fill: var(--color-foreground-level-4);
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
  }

  .outline.disabled:active {
    background-color: rgba(var(--color-background), 0);
    border-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-3);
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
    background-color: var(--color-negative-level-2);
    border-color: var(--color-negative-level-2);
    color: var(--color-background);
  }

  .destructive:active {
    background-color: var(--color-negative-level-2);
    border-color: var(--color-negative-level-2);
    color: var(--color-negative-level-1);
  }

  .destructive:active :global(svg) {
    fill: var(--color-negative-level-1);
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
  }

  .destructive.disabled :global(svg) {
    fill: var(--color-background);
  }

  .destructive.disabled:active {
    background-color: var(--color-negative-level-1);
    border-color: var(--color-negative-level-1);
    color: var(--color-background);
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
  on:click
  on:mouseenter
  on:mouseleave
  {style}>
  <svelte:component this={icon} />
  <div style={icon ? 'margin-left: 8px;' : ''}>
    <slot />
  </div>
</button>
