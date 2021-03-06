<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { get } from "svelte/store";

  import { Button, Dai, Emoji, TextInput, TxButton } from "ui/DesignSystem";

  import * as modal from "ui/src/modal";
  import {
    amountStore,
    balanceValidationStore,
    store,
  } from "ui/src/funding/pool";
  import { ValidationStatus } from "../../../src/validation";

  import Big from "big.js";

  // Validate the amount beign withdrawn
  let validatingAmount = false;
  let amount = "";
  let balance = Big(0);
  let validation = balanceValidationStore(balance);

  $: {
    balance = $store?.data.unwrap()?.balance || balance;
    validation = balanceValidationStore(balance);
  }

  $: amountStore.set(amount);
  $: {
    if ($amountStore && $amountStore.length > 0) {
      validatingAmount = true;
    }
    if (validatingAmount) {
      validation.validate($amountStore);
    }
  }

  $: disableAmountConfirmation =
    $validation && $validation.status !== ValidationStatus.Success;

  async function onCancel(): Promise<void> {
    modal.hide();
  }

  async function onConfirmed(): Promise<void> {
    const pool = get(store);
    if (pool) {
      if (mode === Mode.SpecifyAmount) {
        await pool.withdraw(Big(amount));
      } else {
        await pool.withdrawAll();
      }
      modal.hide();
    }
  }

  enum Mode {
    // The user is specifying the specific amount they want to cashout.
    SpecifyAmount,
    // The user is opting to cashout everything and stop their support.
    CashoutAll,
  }

  let mode = Mode.SpecifyAmount;
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
  p,
  .note,
  .input {
    margin-top: 1.5rem;
  }

  .note {
    font-size: 0.875rem;
    line-height: 1.125rem;
    display: flex;
    align-items: center;
    text-align: center;
    color: var(--color-foreground-level-5);
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: var(--content-padding);
  }
</style>

<div class="wrapper" data-cy="pool-withdraw-modal">
  <Emoji emoji="💸" size="huge" />
  <h1>Cash out</h1>

  {#if mode === Mode.SpecifyAmount}
    <p>
      Enter the amount you’d like to transfer to your linked Ethereum account
      below.
      <!-- svelte-ignore a11y-missing-attribute -->
      <a class="typo-link" on:click={() => (mode = Mode.CashoutAll)}
        >Want to stop support completely</a
      >?
    </p>
    <div class="input">
      <TextInput
        dataCy="modal-amount-input"
        bind:value={amount}
        validation={$validation}
        showLeftItem
        autofocus
        style={"width: 125px"}>
        <div slot="left" style="position: absolute; top: 1px; left: 12px;">
          <Dai />
        </div>
      </TextInput>
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
        disabled={disableAmountConfirmation}
        errorLabel="Failed to withdraw funds">
        Confirm in your wallet
      </TxButton>
    </div>
  {:else}
    <p>Stop support and transfer the entire remaining balance out.</p>

    <div class="note">
      Note: due to the nature of streaming digital money, the amount transferred
      to your linked account will be a bit less than your current balance.
    </div>

    <div class="submit">
      <Button
        variant="transparent"
        dataCy="back"
        on:click={() => (mode = Mode.SpecifyAmount)}
        style="margin-right: 1rem">
        Back
      </Button>

      <TxButton
        variant="destructive"
        onClick={onConfirmed}
        errorLabel="Failed withdraw">
        Stop support and cash out everything
      </TxButton>
    </div>
  {/if}
</div>
