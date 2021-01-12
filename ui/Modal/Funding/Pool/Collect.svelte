<script lang="ts">
  import { resolve } from "path";
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import { Dai, Remote, TxButton } from "../../../DesignSystem/Component";
  import { Button, Emoji } from "../../../DesignSystem/Primitive";

  import * as modal from "../../../src/modal";
  import { store } from "../../../src/funding/pool";

  if ($store === null) pop();
  $: pool = get(store);

  async function onConfirmed(): Promise<void> {
    await $store?.collect();
    modal.hide();
    resolve();
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

    width: 600px;
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
          <Dai style="margin: 0 7px">{poolData.collectableFunds}</Dai>
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
          title={'Confirm in your wallet'}
          errorMessage={e => `Failed to collect incoming support: ${e.message}`} />
      </div>
    </div>
  </Remote>
{/if}
