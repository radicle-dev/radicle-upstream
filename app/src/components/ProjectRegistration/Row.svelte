<script>
  export let style = null;

  export let active = false;
  export let disabled = false;

  $: disabledClass = disabled ? "disabled" : null;
  $: activeClass = active ? "active" : null;
  $: rowClass = ["row", activeClass, disabledClass].join(" ");
</script>

<style>
  .row {
    padding: 18px 16px 18px 16px;
    border: 1px solid var(--color-lightgray);
    border-radius: 4px;
    display: flex;
    align-items: center;
  }

  .row:hover {
    outline: none;
    box-shadow: 0 0 0 1px var(--focus-outline-color, var(--color-pink));
    border: 1px solid var(--focus-outline-color, var(--color-pink));
    cursor: pointer;
  }

  .row.disabled {
    box-shadow: none;
    border: 1px solid var(--color-lightgray);
    cursor: default;
  }

  .row.active {
    box-shadow: 0 0 0 1px var(--focus-outline-color, var(--color-pink));
    border: 1px solid var(--focus-outline-color, var(--color-pink));
  }

  .align-left {
    display: flex;
    flex: 1;
    align-items: center;
  }

  .align-right {
    display: flex;
    flex: 1;
    justify-content: flex-end;
  }
</style>

<div class={rowClass} {style}>
  <div class="align-left">
    <slot name="left" />
  </div>

  <div class="align-right">
    <slot name="right" />
  </div>
</div>
