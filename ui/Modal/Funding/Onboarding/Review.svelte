<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Button, Modal, TxButton } from "ui/DesignSystem";

  import Receivers from "ui/DesignSystem/Funding/Pool/Receivers.svelte";

  import type { Receivers as PoolReceivers } from "ui/src/funding/pool";

  export let onBack: () => void;
  export let onConfirmed: () => Promise<void>;

  export let budget = "";
  export let topUp = "";
  export let receivers: PoolReceivers;
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
  }
  p {
    color: var(--color-foreground-level-6);
    margin-bottom: 1rem;
  }
</style>

<Modal emoji="💸" title="Stream digital money">
  <div class="wrapper">
    <p>
      {#if receivers.size === 0}
        Top up
        <strong>{topUp} DAI</strong>. You haven’t added any receivers yet, but
        as soon as you do, money will begin streaming to them at a rate of
        <strong>{budget} DAI</strong>
        per week.
      {:else}
        Top up
        <strong>{topUp} DAI</strong>
        and stream
        <strong>{budget} DAI</strong>
        per week to these users:
      {/if}
    </p>
  </div>

  <div class="wrapper">
    <Receivers {receivers} />
  </div>

  <svelte:fragment slot="buttons">
    <Button variant="transparent" dataCy="back" on:click={onBack}>Back</Button>

    <TxButton
      dataCy="confirm-button"
      onClick={onConfirmed}
      errorLabel="Failed to onboard your pool">
      Confirm in your wallet
    </TxButton>
  </svelte:fragment>
</Modal>
