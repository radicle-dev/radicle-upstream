<script>
  export let dataCy = null;
  export let id = null;

  const topbar = $$props.$$slots.topbar;
</script>

<style>
  header {
    background: var(--color-background);
    position: sticky;
    top: 0;
  }

  .topbar {
    height: calc(100vh - var(--topbar-height));
    overflow-x: hidden;
    overflow-y: auto;
  }
</style>

{#if topbar}
  <header>
    <slot name="topbar" />
  </header>
{/if}

<div class:topbar {id} data-cy={dataCy}>
  <slot />
</div>
