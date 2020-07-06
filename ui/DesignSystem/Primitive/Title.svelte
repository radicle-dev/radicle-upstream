<script>
  export let dataCy = null;
  export let style = null;
  export let truncate = false;
  export let variant = "regular"; // regular | medium | large | big | huge

  const titleClass = `${variant} ${truncate ? " truncate" : ""}`;
</script>

<style>
  .huge {
    font-family: var(--typeface-bold);
    font-size: 36px;
    line-height: 43px;
  }

  .big {
    font-family: var(--typeface-bold);
    font-size: 24px;
    line-height: 29px;
  }

  .large {
    font-family: var(--typeface-bold);
    font-size: 20px;
    line-height: 24px;
  }

  .medium {
    font-family: var(--typeface-bold);
    font-size: 16px;
    line-height: 22px;
  }

  .regular {
    font-family: var(--typeface-medium);
    font-size: 16px;
    line-height: 22px;
  }

  .truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>

{#if variant === 'huge'}
  <h1 data-cy={dataCy} class={titleClass} {style}>
    <slot />
  </h1>
{:else if variant === 'big'}
  <h2 data-cy={dataCy} class={titleClass} {style}>
    <slot />
  </h2>
{:else}
  <h3 data-cy={dataCy} class={titleClass} {style}>
    <slot />
  </h3>
{/if}
