<script>
  export let style = null;

  // hugeTitle | bigTitle | title |
  // text | smallText | smallTextBold |
  // code |
  // caption |
  // bigNumber | number | smallNumber
  export let variant = "text";
</script>

<style>
  .hugeTitle {
    font-family: var(--typeface-bold);
    font-size: 36px;
    line-height: 43px;
  }

  .bigTitle {
    font-family: var(--typeface-bold);
    font-size: 24px;
    line-height: 29px;
  }

  .title {
    font-family: var(--typeface-medium);
    font-size: 16px;
    line-height: 130%;
  }

  .text {
    font-family: var(--typeface-regular);
    font-size: 16px;
    line-height: 24px;
  }

  .smallText {
    font-family: var(--typeface-regular);
    font-size: 14px;
    line-height: 20px;
  }

  .smallTextBold {
    font-family: var(--typeface-medium);
    font-size: 14px;
    line-height: 20px;
  }

  .code {
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
  }

  .caption {
    font-family: var(--typeface-medium);
    font-size: 13px;
    line-height: 16px;
    letter-spacing: 0.1em;
    text-transform: uppercase;
  }

  .bigNumber {
    font-family: var(--typeface-mono-bold);
    font-size: 22px;
    line-height: 27px;
  }

  .number {
    font-family: var(--typeface-mono-medium);
    font-size: 16px;
    line-height: 19px;
  }

  .smallNumber {
    font-family: var(--typeface-mono-medium);
    font-size: 12px;
    line-height: 14px;
  }
</style>

{#if variant === 'code'}
  <pre class={variant} {style}>
    <slot />
  </pre>
{:else if variant === 'hugeTitle'}
  <h1 class={variant} {style}>
    <slot />
  </h1>
{:else if variant === 'bigTitle'}
  <h2 class={variant} {style}>
    <slot />
  </h2>
{:else if variant === 'title'}
  <h3 class={variant} {style}>
    <slot />
  </h3>
{:else}
  <p class={variant} {style}>
    <slot />
  </p>
{/if}
