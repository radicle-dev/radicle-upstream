<script>
  import { Text } from "../DesignSystem";

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
  <Text variant="caption">Step</Text>
  {#each range as stepNumber}
    {#if step === stepNumber}
      <Text variant="caption">&nbsp;{stepNumber}</Text>
    {:else}
      <Text variant="caption" style="color: var(--color-gray)">
        &nbsp;{stepNumber}
      </Text>
    {/if}
  {/each}
</div>
