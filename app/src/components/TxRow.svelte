<script>
  import RowLayout from "../layouts/helpers/Row.svelte";

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
</style>

<div class={rowClass} {style}>
  <RowLayout>
    <div slot="left">
      <slot name="left" />
    </div>

    <div slot="right">
      <slot name="right" />
    </div>
  </RowLayout>
</div>
