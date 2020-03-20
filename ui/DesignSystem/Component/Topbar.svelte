<script>
  export let style = null;
</script>

<style>
  .topbar {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    background-color: var(--color-white);
    width: calc(100vw - var(--sidebar-width));
    height: var(--topbar-height);
    left: var(--sidebar-width);
    border-bottom: 1px solid var(--color-lightgray);
  }
</style>

<div data-cy="topbar" class="topbar" {style}>
  <slot name="left" />
  <slot name="middle" />
  <slot name="right" />
</div>
