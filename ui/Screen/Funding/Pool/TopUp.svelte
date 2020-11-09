<script lang="ts">
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import { Dai, TxButton } from "../../../DesignSystem/Component";
  import { Button, Input } from "../../../DesignSystem/Primitive";
  import { resolve } from "path";

  import * as modal from "../../../src/modal";
  import {
    amountStore,
    topUpAmountValidationStore,
    store,
  } from "../../../src/funding/pool";
  import { ValidationStatus } from "../../../src/validation";

  if ($store === null) pop();

  let validatingAmount = false;
  let amount: number;

  const amountValidation = topUpAmountValidationStore(
    get(store).getAccount().balance
  );
  $: amountStore.set(amount ? amount.toString() : "");
  $: {
    if ($amountStore && $amountStore.length > 0) validatingAmount = true;
    if (validatingAmount) amountValidation.validate($amountStore);
  }
  $: disableConfirmation =
    $amountValidation && $amountValidation.status !== ValidationStatus.Success;

  async function onConfirmed(): Promise<void> {
    await get(store).topUp(amount);
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
    justify-content: center;
    flex-direction: column;
    padding: var(--content-padding);
    width: 650px;
    background: var(--color-background);
    border-radius: 0.5rem;
  }

  header {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    padding: var(--content-padding);
    margin-bottom: 1.5rem;
    background-color: var(--color-foreground-level-1);
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.25rem;
  }

  h2 {
    margin-top: calc(var(--content-padding) / 2);
  }

  .from-to {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 1rem;
  }

  .sub-section {
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    margin-top: 1.7rem;
  }

  .sub-section p {
    color: var(--color-foreground-level-6);
  }

  .subheading {
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    margin-top: 2rem;
  }
</style>

<div class="wrapper" data-cy="send-funds-modal">
  <div data-cy="preparation-step">
    <header>
      <h2>Top up</h2>

      <div class="from-to">
        <p class="typo-text-bold subheading">Your external wallet</p>
        <p class="typo-text-bold subheading">-&gt;</p>
        <p class="typo-text-bold subheading">Outgoing support</p>
      </div>
    </header>

    <div class="sub-section">
      <p class="typo-text-bold subheading">Amount</p>
      <Input.Text
        dataCy="modal-amount-input"
        placeholder="Enter the amount"
        bind:value={amount}
        showLeftItem
        autofocus
        validation={$amountValidation}>
        <div slot="left" style="position: absolute; top: 1px; left: 12px;">
          <Dai />
        </div>
      </Input.Text>
    </div>

    <div class="submit">
      <Button variant="transparent" dataCy="cancel-topup" on:click={onCancel}>
        Cancel
      </Button>

      <TxButton
        title="Confirm"
        disabled={disableConfirmation}
        dataCy="review-transfer-button"
        onClick={onConfirmed}
        errorMessage={e => `Could not top up pool funds: ${e.message}`} />
    </div>
  </div>
</div>
