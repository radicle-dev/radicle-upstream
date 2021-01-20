<script lang="typescript">
  import type { Tx } from "../../../../src/transaction";

  export let tx: Tx;
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
    {tx.kind}
  </p>
  <p
    class="typo-text-small"
    style="color: var(--color-foreground-level-5); text-transform: capitalize;">
    {tx.hash.slice(0, 9)}
  </p>
</div>
