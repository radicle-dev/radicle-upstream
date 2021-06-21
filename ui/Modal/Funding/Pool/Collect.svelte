<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Button, Dai, Emoji, Remote, TxButton } from "ui/DesignSystem";

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
  .wrapper {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-direction: column;

    text-align: center;

    padding: var(--content-padding);
    background: var(--color-background);
    border-radius: 0.5rem;

    width: 37.5rem;
  }

  h1,
  .description {
    margin-top: 1.5rem;
  }

  .description {
    display: flex;
    align-items: center;
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: var(--content-padding);
  }
</style>

{#if pool}
  <Remote store={pool.data} let:data={poolData}>
    <div class="wrapper" data-cy="collect-incoming-support-modal">
      <Emoji emoji="💸" size="huge" />
      <h1>Collect</h1>

      <div class="typo-text description">
        Collect the
        <div class="typo-text-bold">
          <Dai style="margin: 0 0.4375rem">{poolData.collectableFunds}</Dai>
        </div>
        waiting on you from supporters.
      </div>

      <div class="submit">
        <Button
          variant="transparent"
          dataCy="cancel"
          on:click={onCancel}
          style="margin-right: 1rem">
          Cancel
        </Button>

        <TxButton
          onClick={onConfirmed}
          errorLabel="Failed to collect incoming support">
          Confirm in your wallet
        </TxButton>
      </div>
    </div>
  </Remote>
{/if}
