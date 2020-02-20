<script>
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

  .align-right {
    display: flex;
    flex: 1;
    justify-content: flex-end;
  }
</style>

{#if align !== null}
  <div class="row" {style}>
    <div class={`align-${align}`}>
      <slot />
    </div>
  </div>
{:else}
  <div class="row" {style}>
    <div class="align-left">
      <slot name="left" />
    </div>

    <div class="align-right">
      <slot name="right" />
    </div>
  </div>
{/if}
