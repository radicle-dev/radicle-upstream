<script>
  export let style = null;
  export let size = null;
</script>

<style>
  h3 {
    font-family: "GT America Medium";
    font-size: 16px;
    line-height: 130%;
  }

  h2 {
    font-family: "GT America Bold";
    font-size: 24px;
    line-height: 29px;
  }

  h1 {
    font-family: "GT America Bold";
    font-size: 36px;
    line-height: 43px;
  }
</style>

{#if size === "big"}
  <h2 {style}>
    <slot />
  </h2>
{:else if size === "huge"}
  <h1 {style}>
    <slot />
  </h1>
{:else}
  <h3 {style}>
    <slot />
  </h3>
{/if}
