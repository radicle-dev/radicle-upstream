<script lang="ts">
  import { Button, Icon, Input } from "../../../../Primitive";
  import { Dai, Remote, TxButton } from "../../../../Component";

  import Receivers from "../Receivers.svelte";

  import * as modal from "../../../../../src/modal";
  import * as path from "../../../../../src/path";
  import * as remote from "../../../../../src/remote";
  import * as _pool from "../../../../../src/funding/pool";
  import * as transaction from "../../../../../src/transaction";

  import {
    budgetStore,
    monthlyContributionValidationStore,
  } from "../../../../../src/funding/pool";
  import { ValidationStatus } from "../../../../../src/validation";

  export let pool: _pool.Pool;

  $: _pool.store.set(pool);

  let ongoingTopUp = false;
  let ongoingWithdraw = false;
  let ongoingSupportUpdate = false;
  let paused = false;

  transaction.store.subscribe(_ => {
    ongoingTopUp = transaction.ongoing(transaction.TxKind.TopUp);
    ongoingWithdraw = transaction.ongoing(transaction.TxKind.Withdraw);
    ongoingSupportUpdate = transaction.ongoing(
      transaction.TxKind.UpdateSupport
    );
  });

  // Editing values
  let budget = "";
  let receivers: _pool.Receivers = new Map();

  let validatingBudget = false;
  $: budgetValidation = monthlyContributionValidationStore();
  $: budgetStore.set(budget ? budget.toString() : "");
  $: {
    if ($budgetStore && $budgetStore.length > 0) validatingBudget = true;
    if (validatingBudget) budgetValidation.validate($budgetStore);
  }

  let data: _pool.PoolData;
  pool.data.subscribe(store => {
    if (store.status === remote.Status.Success) {
      data = store.data;
      budget = data.amountPerBlock.toString();
      receivers = new Map(data.receivers);
      paused = data.balance <= data.amountPerBlock || data.amountPerBlock === 0;
    }
  });

  // Flags whether the view is in editing mode.
  // Triggered by the user.
  let editing = false;

  function enterEditMode(): void {
    editing = true;
  }

  $: thereAreChanges =
    budget !== data.amountPerBlock ||
    receivers.size !== data.receivers.size ||
    [...receivers.entries()].find(
      ([address, weight]) => data.receivers.get(address) !== weight
    );

  function leaveEditMode(): void {
    editing = false;
    budget = data.amountPerBlock.toString();
    receivers = new Map(data.receivers);
  }

  function onConfirmInWallet(): Promise<void> {
    console.log("onConfirmInWallet");
    console.log(`onConfirmInWallet, receivers ${[...receivers.entries()]}`);

    const changedReceivers = new Map(
      [...receivers].filter(
        ([address, weight]) => data.receivers.get(address) !== weight
      )
    );

    return pool
      .updateSettings(budget, changedReceivers)
      .then(_ => leaveEditMode());
  }

  const openTopUp = () => {
    _pool.store.set(pool);
    modal.toggle(path.poolTopUp());
  };

  const openWithdrawModal = () => {
    _pool.store.set(pool);
    modal.toggle(path.poolWithdraw());
  };
</script>

<style>
  .container {
    margin: 0;
    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 8px;
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    /* We need to cut enough vertical padding to have the 'Support' heading
    aligned with the 'Balance' heading of the `Panel` component shown at the left*/
    padding: calc(var(--content-padding) - 6px) var(--content-padding);
    border-bottom: 1px solid var(--color-foreground-level-2);
  }
  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  strong {
    font-weight: bold;
    margin: 0 5px;
  }
  .content {
    padding: var(--content-padding);
  }

  .description {
    display: inline-flex;
  }

  .tip {
    font-size: 14px;
    line-height: 18px;
    display: flex;
    gap: 10px;
    align-items: center;
    text-align: left;

    margin-top: var(--content-padding);
    color: var(--color-foreground-level-5);
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: calc(var(--content-padding) / 2);
  }
</style>

<Remote store={pool.data} let:data={poolData}>
  <div class="container">
    <header>
      <div class="row">
        <h3>Support</h3>
        <span class="row" style="margin-left: 14px">
          {#if editing}
            <Input.Text
              dataCy="budget-input"
              bind:value={budget}
              validation={$budgetValidation}
              validationStyle={'position: absolute; top: 30px;'}
              showLeftItem
              autofocus
              style={'width: 125px'}>
              <div
                slot="left"
                style="position: absolute; top: 1px; left: 12px;">
                <Dai />
              </div>
            </Input.Text>
          {:else}
            <Dai>{poolData.amountPerBlock}</Dai>
          {/if}
          <span style="margin-left: 7px;"> per month</span>
        </span>
        <!-- svelte-ignore a11y-missing-attribute -->
        <a
          hidden={editing}
          class="typo-link"
          disabled={ongoingSupportUpdate}
          on:click={enterEditMode}
          style="margin-left: 12px;">
          Edit
        </a>
      </div>
      <div class="row">
        <p>Remaining</p>
        <p class="typo-text-bold row" style="margin-left: 12px">
          <Dai>{poolData.balance}</Dai>
        </p>
        {#if !ongoingWithdraw && !ongoingTopUp}
          <Button
            disabled={poolData.balance === 0}
            dataCy="drain-pool-button"
            variant="transparent"
            on:click={openWithdrawModal}
            style="margin-left: 12px">
            Withdraw
          </Button>
        {/if}
        {#if !ongoingTopUp}
          <Button
            dataCy="top-up-pool-button"
            variant="vanilla"
            on:click={openTopUp}
            style="margin-left: 12px">
            Top up
          </Button>
        {/if}
      </div>
    </header>

    <div class="content">
      <div class="description">
        {#if poolData.receivers.size === 0}
          <p>
            Add receivers to your outgoing support by clicking the “Support”
            button on user profiles. You can also add any Ethereum address to
            your Stream.
            <!-- svelte-ignore a11y-missing-attribute -->
            <a
              hidden={editing}
              class="typo-link"
              disabled={ongoingSupportUpdate}
              style="margin-left: 5px;"
              on:click={enterEditMode}>
              Edit
            </a>
          </p>
        {:else}
          <div style="display: flex; align-items: center">
            <strong style="margin-left: 0px">
              <Dai>{poolData.amountPerBlock}</Dai></strong>
            per month will go to each of the
            <strong>{poolData.receivers.size} </strong>
            receivers you're supporting.

            <!-- svelte-ignore a11y-missing-attribute -->
            <a
              hidden={editing}
              class="typo-link"
              disabled={ongoingSupportUpdate}
              style="margin-left: 5px;"
              on:click={enterEditMode}>
              Edit
            </a>
          </div>
        {/if}
      </div>

      <Receivers
        {editing}
        bind:receivers
        updating={ongoingSupportUpdate}
        style="margin-top: 1rem" />

      <div class="tip">
        {#if paused}
          <Icon.ExclamationCircle />Your support is paused. To resume, make sure
          you have enough balance and a montly budget set to be split amongst
          the users you support.
        {:else}
          <Icon.InfoCircle />
          To stop or pause your support, set the monthly contribution to 0 or
          withdraw all the remaining balance.
        {/if}
      </div>
    </div>
  </div>

  {#if editing}
    <div class="submit">
      <Button
        variant="transparent"
        dataCy="cancel"
        on:click={leaveEditMode}
        style="margin-right: 1rem">
        {thereAreChanges ? 'Discard changes' : 'Cancel'}
      </Button>

      <TxButton
        title={'Confirm in your wallet'}
        disabled={!thereAreChanges || $budgetValidation.status !== ValidationStatus.Success}
        dataCy="confirm-button"
        onClick={onConfirmInWallet}
        errorMessage={e => `Failed to save support settings: ${e.message}`} />
    </div>
  {/if}
</Remote>
