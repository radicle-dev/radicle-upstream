<script lang="typescript">
  import type { Tx } from "../../../src/transaction";
  import { colorForStatus } from "../../../src/transaction";

  export let tx: Tx;

  $: color = colorForStatus(tx.status);
</script>

<style>
  .description {
    align-self: center;
    display: flex;
    flex-direction: column;
  }
</style>

<div class="description">
  <p
    class="typo-text-small-bold"
    style="color: var(--color-foreground-level-6); width: max-content">
    {tx.inner.kind}
  </p>
  <p
    class="typo-text-small"
    style="color: {color}; text-transform: capitalize;">
    {tx.status}
  </p>
</div>
