<script lang="ts">
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import { Copyable, Remote, TxButton } from "../../../DesignSystem/Component";
  import { Button, Icon, Input } from "../../../DesignSystem/Primitive";
  import { resolve } from "path";

  import * as modal from "../../../src/modal";
  import {
    CONTRACT_ADDRESS,
    amountStore,
    displayAddress,
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
    await get(store).cashout(amount);
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

  header .from-to .address,
  header .from-to .balance {
    display: flex;
    justify-content: center;
    align-items: center;
    color: var(--color-foreground-level-5);
    font-size: 14px;
    text-align: center;
  }
</style>

<div class="wrapper" data-cy="send-funds-modal">
  <div data-cy="preparation-step">
    <header>
      <h2>Cash out</h2>

      <div class="from-to">
        <div class="from typo-text-bold subheading">
          <p class="typo-text-bold">Outgoing support</p>
          <div class="address">
            <Copyable
              showIcon={false}
              styleContent={false}
              copyContent={CONTRACT_ADDRESS}
              notificationText="Address copied to the clipboard">
              {displayAddress(CONTRACT_ADDRESS)}
            </Copyable>
          </div>
          <div class="balance">
            <Remote store={get(store).data} let:data={poolData}>
              <Icon.CurrencyDAI
                style="fill: var(--color-foreground-level-5); padding-top: 3px; width: 20px; height: 20px" />
              {poolData.balance}
            </Remote>
          </div>
        </div>
        <div class="typo-text-bold subheading arrow">-&gt;</div>
        <div class="typo-text-bold subheading">
          <p class="typo-text-bold">Your connected wallet</p>
          <div class="address">
            <Copyable
              showIcon={false}
              styleContent={false}
              copyContent={get(store).getAccount().address}
              notificationText="Address copied to the clipboard">
              {displayAddress(get(store).getAccount().address)}
            </Copyable>
          </div>
          <div class="balance">
            <Icon.CurrencyDAI
              style="fill: var(--color-foreground-level-5); padding-top: 3px; width: 20px; height: 20px" />
            {get(store).getAccount().balance}
          </div>
        </div>
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
        <div slot="left" style="position: absolute; top: 2px; left: 10px;">
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
        disabled={disableConfirmation}
        dataCy="review-transfer-button"
        onClick={onConfirmed}
        errorMessage={e => `Could not cashout pool funds: ${e.message}`} />
    </div>
  </div>
</div>
