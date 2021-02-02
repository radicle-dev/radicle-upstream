<style>
  #footer {
    display: flex;
    align-items: flex-end;
    justify-content: flex-end;
    gap: var(--content-padding);

    position: fixed;
    bottom: var(--content-padding);
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
    padding: 0 var(--content-padding);
    z-index: 1001;
  }
</style>

<div id="footer">
  <slot />
</div>
