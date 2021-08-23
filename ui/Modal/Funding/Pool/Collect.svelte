<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Button, Dai, Modal, Remote, TxButton } from "ui/DesignSystem";

  import * as modal from "ui/src/modal";
  import { store } from "ui/src/funding/pool";

  $: pool = $store;

  async function onConfirmed(): Promise<void> {
    await $store?.collect();
    modal.hide();
  }

  async function onCancel(): Promise<void> {
    modal.hide();
  }
</script>

<style>
  .description {
    display: flex;
    justify-content: center;
  }
</style>

{#if pool}
  <Modal emoji="💸" title="Collect">
    <Remote store={pool.data} let:data={poolData}>
      <div class="typo-text description">
        Collect the
        <div class="typo-text-bold">
          <Dai style="margin: 0 0.4375rem">{poolData.collectableFunds}</Dai>
        </div>
        waiting on you from supporters.
      </div>
    </Remote>

    <svelte:fragment slot="buttons">
      <Button variant="transparent" dataCy="cancel" on:click={onCancel}>
        Cancel
      </Button>

      <TxButton
        onClick={onConfirmed}
        errorLabel="Failed to collect incoming support">
        Confirm in your wallet
      </TxButton>
    </svelte:fragment>
  </Modal>
{/if}
