<script lang="ts">
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import {
    amountStore,
    amountValidationStore,
    store,
  } from "../../../src/funding/pool";
  import { ValidationStatus } from "../../../src/validation";

  import { ModalLayout, StatefulButton } from "../../../DesignSystem/Component";
  import { Icon, Input } from "../../../DesignSystem/Primitive";
  import { resolve } from "path";

  if ($store === null) pop();

  let validatingAmount = false;
  let amount: number;

  $: amountStore.set(amount ? amount.toString() : "");
  $: {
    if ($amountStore && $amountStore.length > 0) validatingAmount = true;
    if (validatingAmount) amountValidationStore.validate($amountStore);
  }
  $: disableConfirmation =
    $amountValidationStore &&
    $amountValidationStore.status !== ValidationStatus.Success;

  async function onConfirmed(): Promise<void> {
    const pool = get(store);
    await pool.topUp(amount);
    pop();
    resolve();
  }
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
        <h2>Top up your pool 😉</h2>
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
        showLeftItem
        autofocus
        validation={$amountValidationStore}>
        <div slot="left" style="position: absolute; top: 9px; left: 10px;">
          <Icon.CurrencyDAI style="fill: var(--color-foreground-level-6)" />
        </div>
      </Input.Text>

      <div class="submit">
        <StatefulButton
          title="Confirm"
          disabled={disableConfirmation}
          dataCy="review-transfer-button"
          onClick={onConfirmed}
          successMessage={`Successfully added eth ${amount} to your pool`}
          errorMessage={e => `Could not top up pool funds: ${e.message}`} />
      </div>
    </div>
  </div>
</ModalLayout>
