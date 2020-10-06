<script lang="ts">
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import { store } from "../src/funding/pool";
  import * as notification from "../src/notification";
  import {
    TransferState,
    amountStore,
    amountValidationStore,
  } from "../src/transfer";

  import { ModalLayout } from "../DesignSystem/Component";
  import { Button, Icon, Input } from "../DesignSystem/Primitive";

  if ($store === null) pop();

  let validatingAmount = false;
  let state = TransferState.Preparation;
  let amount: number = 0;

  $: disableForm = state !== TransferState.Preparation;
  $: amountValidation = amountValidationStore("TODO(nuno)", amount);
  $: amountStore.set(amount.toString());
  $: {
    if ($amountStore && $amountStore.length > 0) validatingAmount = true;
    if (validatingAmount) amountValidation.validate($amountStore);
  }

  const onConfirmed = async () => {
    try {
      state = TransferState.Confirmation;
      const pool = get(store);
      const result = await pool.topUp(amount);
      console.log("OnConfirmed result", result);
    } catch (error) {
      notification.error(`Could not transfer funds: ${error.message}`);
    }
  };
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    flex-direction: column;
    margin: 92px 0 32px 0;
    width: 540px;
  }
  header {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    padding: 2rem;
    margin-bottom: 1.5rem;
    background-color: var(--color-foreground-level-1);
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.25rem;
  }
  .icon {
    height: 2.5rem;
    width: 2.5rem;
    border-radius: 1.25rem;
    background-color: var(--color-primary-level-1);
    display: flex;
    justify-content: center;
    align-items: center;
    margin-bottom: 1rem;
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    padding-top: 1.5rem;
  }

  .from-to {
    display: grid;
    grid-template-columns: 13rem 1.5rem 13rem;
    grid-column-gap: 1rem;
    margin-top: 1rem;
  }

  .from {
    display: flex;
    justify-content: flex-end;
  }
</style>

<ModalLayout dataCy="page">
  <div class="wrapper" data-cy="send-funds-modal">
    <div data-cy="preparation-step">
      <header>
        <div class="icon">
          <Icon.ArrowUp style="fill: var(--color-primary)" />
        </div>
        <h2>Fill up your pool 😉</h2>
      </header>
      <p
        class="typo-text-bold"
        style="color: var(--color-foreground-level-6); padding: 0.5rem;">
        Amount
      </p>
      <Input.Text
        dataCy="modal-amount-input"
        placeholder="Enter the amount"
        bind:value={amount}
        disabled={disableForm}
        showLeftItem
        autofocus
        style="flex: 1; padding-bottom: 0.5rem;"
        validation={$amountValidation}>
        <div slot="left" style="display: flex;">
          <Icon.CurrencyRAD style="fill: var(--color-foreground-level-6)" />
        </div>
      </Input.Text>

      <div class="submit">
        <Button
          dataCy="review-transfer-button"
          disabled={disableForm}
          on:click={onConfirmed}>
          Confirm
        </Button>
      </div>
      <div style="text-align: center">
        {#if disableForm}
          <em>📱 👀 Approve the transaction on your wallet app...</em>
        {/if}
      </div>
    </div>
  </div>
</ModalLayout>
