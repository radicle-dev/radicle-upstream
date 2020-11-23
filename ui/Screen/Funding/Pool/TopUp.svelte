<script lang="ts">
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import { TxButton } from "../../../DesignSystem/Component";
  import TopUp from "../../../DesignSystem/Component/Funding/Pool/Outgoing/TopUp.svelte";

  import { resolve } from "path";
  import * as modal from "../../../src/modal";
  import { store } from "../../../src/funding/pool";

  if ($store === null) pop();

  let amount = 0;
  async function onConfirmed(): Promise<void> {
    await get(store).topUp(amount);
    modal.hide();
    resolve();
  }

  async function onCancel(): Promise<void> {
    modal.hide();
  }

  let disabled = true;
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
    height: 400px;
  }
</style>

<div class="wrapper" data-cy="top-up-modal">
  <TopUp
    bind:amount
    balance={$store.getAccount().balance * 1}
    onBack={['Cancel', onCancel]}
    bind:disabled>
    <TxButton
      onClick={onConfirmed}
      title={'Confirm in your wallet'}
      errorMessage={e => `Failed top up: ${e.message}`}
      {disabled} />
  </TopUp>
</div>
