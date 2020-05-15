<script>
  export let dataCy = null;
  export let style = null;
  export let align = null; // left | right
</script>

<style>
  .row {
    display: flex;
    align-items: center;
  }

  .align-left {
    display: flex;
    flex: 1;
  }

  .align-center {
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .align-right {
    display: flex;
    flex: 1;
    justify-content: flex-end;
  }
</style>

{#if align !== null}
  <div data-cy={dataCy} class={`row align-${align}`} {style}>
    <slot />
  </div>
{:else}
  <div data-cy={dataCy} class="row" {style}>
    <div class="align-left">
      <slot name="left" />
    </div>

    <div class="align-right">
      <slot name="right" />
    </div>
  </div>
{/if}
