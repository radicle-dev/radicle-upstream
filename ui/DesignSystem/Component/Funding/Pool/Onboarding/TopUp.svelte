<script lang="typescript">
  import { Box, Button, Icon } from "../../../../Primitive";

  import * as modal from "../../../../../src/modal";
  import * as path from "../../../../../src/path";

  export let style = "";

  // The balance of this pool.
  export let balance = "";
  // Flag whether there is already an ongoing TopUp transaction.
  export let ongoing = false;

  $: done = balance > 0;

  const openSendModal = () => {
    modal.toggle(path.poolTopUp());
  };
</script>

<style>
  h2,
  p {
    margin-top: 1rem;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  strong {
    font-weight: bold;
  }
</style>

<Box {style} {done}>
  <h2>Top up</h2>
  <p>Fill up your outgoing balance.</p>
  <p style="display: flex; align-items: center;">
    Balance <strong style="display: flex; margin-left: 10px;">
      <Icon.CurrencyDAI
        style="fill: var(--color-foreground-level-6); padding-top: 3px;" />
      {balance}</strong>
  </p>
  {#if !done}
    <Button
      disabled={ongoing}
      dataCy="top-up-pool-button"
      variant="primary"
      on:click={openSendModal}
      style="margin-top: 12px">
      Top up
    </Button>
  {/if}
</Box>
