<script>
  export let variant;
  export let disabled = false;
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
    height: 48px;
    outline-style: none;
    padding: 13px 24px 14px 24px;
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

<button class={variant} {disabled} class:disabled>
  <slot />
</button>
