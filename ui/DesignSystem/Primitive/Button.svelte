<script>
  import Title from "./Title.svelte";

  // vanilla | primary | secondary | transparent | outline | destructive
  export let variant = "primary";

  export let disabled = null;
  export let icon = null;
  export let style = null;
  export let dataCy = null;

  const iconClass = icon ? "icon" : null;

  // we want to dynamically change whether a button is disabled or not
  $: disabledClass = disabled ? "disabled" : null;
  $: buttonClass = [variant, iconClass, disabledClass].join(" ");
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
    height: 40px;
    outline-style: none;
    padding: 0 12px;
    user-select: none;
  }

  .button :global(svg) {
    fill: var(--color-foreground-level-6);
    margin-right: 8px;
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
    cursor: default;
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
    cursor: default;
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
    cursor: default;
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
    background-color: var(--color-background);
    border-color: var(--color-background);
    color: var(--color-foreground-level-6);
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
    background-color: var(--color-background);
    border-color: var(--color-background);
    color: var(--color-foreground-level-3);
    cursor: default;
  }

  .transparent.disabled:hover {
    background-color: var(--color-background);
    box-shadow: 0 0 0 0;
    border-color: var(--color-background);
    color: var(--color-foreground-level-3);
  }

  .transparent.disabled:active {
    background-color: var(--color-background);
    border-color: var(--color-background);
    color: var(--color-foreground-level-3);
  }

  .transparent.disabled :global(svg) {
    fill: var(--color-foreground-level-3);
  }

  .outline {
    background-color: var(--color-background);
    border-color: var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
  }

  .outline :global(svg) {
    fill: var(--color-foreground-level-5);
  }

  .outline:hover {
    background-color: var(--color-foreground-level-1);
    box-shadow: 0 0 0 1px
      var(--focus-outline-color, var(--color-foreground-level-3));
    border: 1px solid var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
  }

  .outline:active {
    background-color: var(--color-foreground-level-1);
    box-shadow: 0 0 0 1px
      var(--focus-outline-color, var(--color-foreground-level-2));
    border-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-5);
  }

  .outline:active :global(svg) {
    fill: var(--color-foreground-level-4);
  }

  .outline.disabled {
    background-color: var(--color-background);
    border-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-3);
    cursor: default;
  }

  .outline.disabled:hover {
    background-color: var(--color-background);
    box-shadow: 0 0 0 0;
    border-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-3);
  }

  .outline.disabled:active {
    background-color: var(--color-background);
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
    cursor: default;
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
</style>

<button
  data-cy={dataCy}
  class="button {buttonClass}"
  {disabled}
  on:click
  {style}>
  <svelte:component this={icon} />
  <Title>
    <slot />
  </Title>
</button>
