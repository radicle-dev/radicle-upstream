<script>
  export let dataCy = null;
  export let style = null;
  export let align = null; // left | right
</script>

<style>
  .row {
    align-items: center;
    display: flex;
  }

  .align-left {
    display: flex;
    flex: 1;
  }

  .align-center {
    align-items: center;
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: center;
  }

  .align-right {
    display: flex;
    flex: 1;
    justify-content: flex-end;
  }
</style>

{#if align !== null}
  <div data-cy={dataCy} class={`row align-${align}`} on:click {style}>
    <slot />
  </div>
{:else}
  <div data-cy={dataCy} class="row" on:click {style}>
    <div class="align-left">
      <slot name="left" />
    </div>

    <div class="align-right">
      <slot name="right" />
    </div>
  </div>
{/if}
