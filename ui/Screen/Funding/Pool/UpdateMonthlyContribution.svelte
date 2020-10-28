<script lang="ts">
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import { TxButton } from "../../../DesignSystem/Component";
  import { Button, Icon, Input } from "../../../DesignSystem/Primitive";
  import { resolve } from "path";

  import * as modal from "../../../src/modal";
  import {
    amountStore,
    monthlyContributionValidationStore,
    store,
  } from "../../../src/funding/pool";
  import { ValidationStatus } from "../../../src/validation";

  if ($store === null) pop();

  let validatingAmount = false;
  let amount: number;

  const amountValidation = monthlyContributionValidationStore();
  $: amountStore.set(amount ? amount.toString() : "");
  $: {
    if ($amountStore && $amountStore.length > 0) validatingAmount = true;
    if (validatingAmount) amountValidation.validate($amountStore);
  }
  $: confirmDisabled =
    !$amountValidation || $amountValidation.status !== ValidationStatus.Success;

  async function onConfirmed(): Promise<void> {
    await get(store).updateAmountPerBlock(amount);
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
  .icon {
    height: 56px;
    width: 56px;
    border-radius: 50%;
    background-color: var(--color-primary-level-5);
    border: 2px solid #5555ff;
    display: flex;
    justify-content: center;
    align-items: center;
    margin-bottom: 1rem;
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

<div class="wrapper" data-cy="set-monthly-contribution-modal">
  <div data-cy="preparation-step">
    <header>
      <div class="icon">
        <Icon.ArrowUp style="fill: #5555FF" />
      </div>
      <h2>Outgoing monthly contribution</h2>

      <div class="from-to">
        <p class="typo-text-bold subheading">Your external wallet</p>
        <p class="typo-text-bold subheading">-&gt;</p>
        <p class="typo-text-bold subheading">Outgoing support</p>
      </div>
    </header>

    <div class="sub-section">
      <p class="typo-text-bold subheading">Monthly contribution</p>
      <Input.Text
        dataCy="modal-amount-input"
        placeholder="Enter the amount"
        bind:value={amount}
        showLeftItem
        autofocus
        validation={$amountValidation}>
        <div slot="left" style="position: absolute; top: 9px; left: 10px;">
          <Icon.CurrencyDAI style="fill: var(--color-foreground-level-6)" />
        </div>
      </Input.Text>
    </div>

    <div class="submit">
      <Button variant="transparent" dataCy="cancel-topup" on:click={onCancel}>
        Cancel
      </Button>

      <TxButton
        title="Confirm"
        disabled={confirmDisabled}
        dataCy="confirm-button"
        onClick={onConfirmed}
        errorMessage={e => `Could not top up pool funds: ${e.message}`} />
    </div>
  </div>
</div>
