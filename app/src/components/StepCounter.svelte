<script>
  import { Caption } from "../DesignSystem";

  export let step = null;
  export let of = null;
  export let style = null;

  let range = [...Array(of)].map((_, i) => ++i);
</script>

<style>
  .step-counter {
    display: inline-flex;
  }
</style>

<div class="step-counter" {style}>
  <Caption>Step</Caption>
  {#each range as stepNumber}
    {#if step === stepNumber}
      <Caption>&nbsp;{stepNumber}</Caption>
    {:else}
      <Caption style="color: var(--color-gray)">&nbsp;{stepNumber}</Caption>
    {/if}
  {/each}
</div>
