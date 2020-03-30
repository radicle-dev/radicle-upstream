<script>
  import Title from "./Title.svelte";

  // vanilla | primary | secondary | transparent | outline
  export let variant = "primary";
  // big | small
  export let size = "big";

  export let disabled = null;
  export let icon = null;
  export let style = null;
  export let dataCy = null;

  const iconClass = icon ? (size === "big" ? "icon" : "icon-small") : null;

  // we want to dynamically change whether a button is disabled or not
  $: disabledClass = disabled ? "disabled" : null;
  $: buttonClass = [variant, size, iconClass, disabledClass].join(" ");
</script>

<style>
  button {
    white-space: nowrap;
    background-color: var(--color-foreground-level-3);
    border-radius: 4px;
    border-style: solid;
    border-width: 1px;
    border-color: var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
    cursor: pointer;
    display: flex;
    align-items: center;
  }

  .big {
    height: 48px;
    outline-style: none;
    padding: 0 24px 0 24px;
  }

  .small {
    height: 36px;
    outline-style: none;
    padding: 0 16px 0 16px;
    border-radius: 2px;
  }

  .icon {
    padding: 0 24px 0 15px;
  }

  .icon-small {
    padding: 0 16px 0 11px;
  }

  button :global(svg) {
    fill: var(--color-foreground-level-6);
    margin-right: 8px;
  }

  .primary :global(svg),
  .secondary :global(svg) {
    fill: var(--color-background);
  }

  .disabled :global(svg) {
    fill: var(--color-foreground-level-5);
  }

  button:hover {
    background-color: var(--color-foreground-level-2);
    border-color: var(--color-foreground-level-2);
  }

  button:active {
    background-color: var(--color-foreground-level-4);
    border-color: var(--color-foreground-level-4);
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
    background-color: var(--color-primary-level-6);
    border-color: var(--color-primary-level-6);
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
    background-color: var(--color-secondary-level-6);
    border-color: var(--color-secondary-level-6);
    color: var(--color-background);
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
    background-color: var(--color-foreground-level-4);
    border-color: var(--color-foreground-level-4);
    color: var(--color-foreground-level-6);
  }

  .outline {
    background-color: var(--color-background);
    border-color: var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
  }

  .outline:hover {
    background-color: var(--color-foreground-level-2);
    border-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
  }

  .outline:active {
    background-color: var(--color-foreground-level-4);
    border-color: var(--color-foreground-level-4);
    color: var(--color-foreground-level-6);
  }

  .disabled {
    background-color: var(--color-foreground-level-3);
    border-color: var(--color-foreground-level-3);
    color: var(--color-foreground-level-5);
  }

  .disabled:hover {
    background-color: var(--color-foreground-level-3);
    border-color: var(--color-foreground-level-3);
    color: var(--color-foreground-level-5);
  }

  .disabled:active {
    background-color: var(--color-foreground-level-3);
    border-color: var(--color-foreground-level-3);
    color: var(--color-foreground-level-5);
  }
</style>

<button data-cy={dataCy} class={buttonClass} {disabled} on:click {style}>
  <svelte:component this={icon} />
  <Title>
    <slot />
  </Title>
</button>
