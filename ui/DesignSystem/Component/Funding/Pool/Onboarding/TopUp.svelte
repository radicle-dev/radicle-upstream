<script lang="typescript">
  import { Box, Button } from "../../../../Primitive";
  import * as modal from "../../../../../src/modal";
  import * as path from "../../../../../src/path";

  export let style = "";
  export let balance = "";

  const openSendModal = () => {
    modal.toggle(path.poolTopUp());
  };
</script>

<style>
  h2,
  p {
    margin-bottom: 1rem;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  strong {
    font-weight: bold;
  }
</style>

<Box {style}>
  <h2>Top up</h2>
  <p>Fill up your outgoing balance.</p>
  <p>Balance <strong>{balance} DAI</strong></p>
  <Button
    dataCy="top-up-pool-button"
    variant="primary"
    on:click={openSendModal}
    style="margin-left: 12px">
    Top up
  </Button>
</Box>
