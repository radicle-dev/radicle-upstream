<script>
  export let style = null;
</script>

<style>
  .topbar {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    background-color: var(--color-background);
    width: calc(100vw - var(--sidebar-width));
    height: var(--topbar-height);
    left: var(--sidebar-width);
    border-bottom: 1px solid var(--color-foreground-level-3);
    z-index: 1;
  }

  .left {
    display: flex;
    align-items: center;
    height: 100%;
    border-right: 1px solid var(--color-foreground-level-3);
    padding-left: 16px;
    padding-right: 24px;
  }

  .right {
    display: flex;
    align-items: center;
    width: 100%;
    justify-content: flex-end;
  }
</style>

<div data-cy="topbar" class="topbar" {style}>
  <div class="left">
    <slot name="left" />
  </div>
  <slot name="middle" />
  <div class="right">
    <slot name="right" />
  </div>
</div>
