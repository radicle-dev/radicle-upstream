<script lang="typescript">
  import { Box } from "../../../../Primitive";

  export let style = "";
  export let done = false;
</script>

<style>
  h2,
  p {
    margin-top: 1rem;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  .tip {
    font-size: 14px;
    line-height: 18px;

    display: flex;
    align-items: center;
    text-align: center;

    color: var(--color-foreground-level-5);
  }
</style>

<Box {style} {done}>
  <h2>Add</h2>
  <p>
    Add receivers to your outgoing support by clicking the “Support” button on
    profiles or project pages.
  </p>
  <p class="tip">
    Tip: Add any Ethereum address to receive outgoing support with the field
    below.
  </p>
</Box>
