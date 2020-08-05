<script>
  export let title = null;
</script>

<style>
  .swatch {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    padding: 2rem 0;
    border-top: 1px solid var(--color-foreground-level-2);
  }

  p {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="swatch">
  <p class="typo-small">{title}</p>
  <slot />
</div>
