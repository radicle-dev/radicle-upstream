<script>
  import Title from "./Title";
  export let variant = "primary";
  export let size = "big";
  export let disabled = false;
  export let icon = null;

  let iconClass = icon ? (size === "big" ? "icon" : "icon-small") : null;
  let disabledClass = disabled ? "disabled" : null;
  let buttonClass = [variant, size, iconClass, disabledClass].join(" ");
</script>

<style>
  button {
    white-space: nowrap;
    background-color: var(--color-lightgray);
    border-radius: 4px;
    border-style: solid;
    border-width: 1px;
    border-color: var(--color-lightgray);
    color: var(--color-darkgray);
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
  }

  .icon {
    padding: 0 24px 0 15px;
  }

  .icon-small {
    padding: 0 16px 0 11px;
  }

  button :global(svg) {
    fill: var(--color-darkgray);
    margin-right: 8px;
  }

  .primary :global(svg),
  .secondary :global(svg) {
    fill: var(--color-white);
  }

  .disabled :global(svg) {
    fill: var(--color-gray);
  }

  button:hover {
    background-color: var(--color-lightgray-tint-10);
    border-color: var(--color-lightgray-tint-10);
  }

  button:active {
    background-color: var(--color-lightgray-shade-10);
    border-color: var(--color-lightgray-shade-10);
  }

  .primary {
    background-color: var(--color-pink);
    border-color: var(--color-pink);
    color: var(--color-white);
  }

  .primary:hover {
    background-color: var(--color-pink-tint-10);
    border-color: var(--color-pink-tint-10);
    color: var(--color-white);
  }

  .primary:active {
    background-color: var(--color-pink-shade-10);
    border-color: var(--color-pink-shade-10);
    color: var(--color-white);
  }

  .secondary {
    background-color: var(--color-purple);
    border-color: var(--color-purple);
    color: var(--color-white);
  }

  .secondary:hover {
    background-color: var(--color-purple-tint-10);
    border-color: var(--color-purple-tint-10);
    color: var(--color-white);
  }

  .secondary:active {
    background-color: var(--color-purple-shade-10);
    border-color: var(--color-purple-shade-10);
    color: var(--color-white);
  }

  .transparent {
    background-color: var(--color-white);
    border-color: var(--color-white);
    color: var(--color-darkgray);
  }

  .transparent:hover {
    background-color: var(--color-lightgray-tint-10);
    border-color: var(--color-lightgray-tint-10);
    color: var(--color-darkgray);
  }

  .transparent:active {
    background-color: var(--color-lightgray-shade-10);
    border-color: var(--color-lightgray-shade-10);
    color: var(--color-darkgray);
  }

  .outline {
    background-color: var(--color-white);
    border-color: var(--color-lightgray);
    color: var(--color-darkgray);
  }

  .outline:hover {
    background-color: var(--color-lightgray-tint-10);
    border-color: var(--color-lightgray-tint-10);
    color: var(--color-darkgray);
  }

  .outline:active {
    background-color: var(--color-lightgray-shade-10);
    border-color: var(--color-lightgray-shade-10);
    color: var(--color-darkgray);
  }

  .disabled {
    background-color: var(--color-lightgray);
    border-color: var(--color-lightgray);
    color: var(--color-gray);
  }

  .disabled:hover {
    background-color: var(--color-lightgray);
    border-color: var(--color-lightgray);
    color: var(--color-gray);
  }

  .disabled:active {
    background-color: var(--color-lightgray);
    border-color: var(--color-lightgray);
    color: var(--color-gray);
  }
</style>

<button class={buttonClass} {disabled}>
  <svelte:component this={icon} />
  <Title.Regular>
    <slot />
  </Title.Regular>
</button>
