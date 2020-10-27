<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import {
    amountStore,
    topUpAmountValidationStore,
    store,
  } from "../../../src/funding/pool";
  import { ValidationStatus } from "../../../src/validation";

  import { TxButton } from "../../../DesignSystem/Component";
  import { Button, Icon, Input } from "../../../DesignSystem/Primitive";
  import { resolve } from "path";

  if ($store === null) pop();

  const dispatch = createEventDispatcher();

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
    dispatch("hide");
    resolve();
  }

  async function onCancel(): Promise<void> {
    dispatch("hide");
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

  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;

    border: 1px solid var(--color-foreground-level-3);
    box-sizing: border-box;
    border-radius: 4px;

    padding: 10px;
  }

  .subheading {
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
  }

  .address {
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;

    display: inline-flex;
    max-width: 80px;
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
      <div class="icon">
        <Icon.ArrowDown style="fill: #5555FF" />
      </div>
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
        <div slot="left" style="position: absolute; top: 9px; left: 10px;">
          <Icon.CurrencyDAI style="fill: var(--color-foreground-level-6)" />
        </div>
      </Input.Text>
    </div>

    <div class="sub-section">
      <p class="typo-text-bold subheading">From</p>
      <div class="row">
        <p>Your external wallet</p>
        <p>
          <span>{get(store).getAccount().balance} DAI <span
              class="address">{get(store).getAccount().address}</span></span>
        </p>
      </div>
    </div>

    <div class="sub-section">
      <p class="typo-text-bold subheading">To</p>
      <div class="row">
        <p>Outgoing support balance</p>
      </div>
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
