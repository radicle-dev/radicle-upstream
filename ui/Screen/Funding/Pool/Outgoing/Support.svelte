<script lang="ts">
  import { Button, Icon, Input } from "../../../../DesignSystem/Primitive";
  import { Dai, Remote, TxButton } from "../../../../DesignSystem/Component";

  import Receivers from "../../../../DesignSystem/Component/Funding/Pool/Receivers.svelte";

  import * as modal from "../../../../src/modal";
  import * as path from "../../../../src/path";
  import * as remote from "../../../../src/remote";
  import * as fundingPool from "../../../../src/funding/pool";
  import * as transaction from "../../../../src/transaction";

  import {
    budgetStore,
    monthlyContributionValidationStore,
  } from "../../../../src/funding/pool";
  import { ValidationStatus } from "../../../../src/validation";

  import { BigNumber } from "ethers";

  export let pool: fundingPool.Pool;

  $: fundingPool.store.set(pool);

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
  let receivers: fundingPool.Receivers = new Map();

  let validatingBudget = false;
  $: budgetValidation = monthlyContributionValidationStore();
  $: budgetStore.set(budget);
  $: {
    if ($budgetStore && $budgetStore.length > 0) validatingBudget = true;
    if (validatingBudget) budgetValidation.validate($budgetStore);
  }

  // Flags whether the view is in editing mode.
  // Triggered by the user.
  let editing = false;

  function enterEditMode(): void {
    editing = true;
  }

  let data: fundingPool.PoolData;
  pool.data.subscribe(store => {
    if (store.status === remote.Status.Success) {
      data = store.data;
      if (!editing) {
        budget = data.amountPerBlock.toString();
        receivers = new Map(data.receivers);
      }
      paused =
        data.balance.lte(data.amountPerBlock) || data.amountPerBlock.isZero();
    }
  });

  $: thereAreChanges =
    fundingPool.isValidBigNumber(budget) &&
    (!BigNumber.from(budget).eq(data.amountPerBlock) ||
      receivers.size !== data.receivers.size ||
      [...receivers.entries()].find(
        ([address, weight]) => data.receivers.get(address) !== weight
      ));

  function leaveEditMode(): void {
    editing = false;
    budget = data.amountPerBlock.toString();
    receivers = new Map(data.receivers);
  }

  async function onConfirmInWallet(): Promise<void> {
    const changedReceivers = new Map(
      [...receivers].filter(
        ([address, weight]) => data.receivers.get(address) !== weight
      )
    );

    return pool
      .updateSettings(BigNumber.from(budget), changedReceivers)
      .then(_ => leaveEditMode());
  }

  const openTopUp = () => {
    fundingPool.store.set(pool);
    modal.toggle(path.poolTopUp());
  };

  const openWithdrawModal = () => {
    fundingPool.store.set(pool);
    modal.toggle(path.poolWithdraw());
  };
</script>

<style>
  .container {
    margin: 0;
    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 0.5rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    /* We need to cut enough vertical padding to have the 'Support' heading
    aligned with the 'Balance' heading of the `Panel` component shown at the left*/
    padding: calc(var(--content-padding) - 0.375rem) var(--content-padding);
    border-bottom: 1px solid var(--color-foreground-level-2);
  }

  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  strong {
    font-weight: bold;
    margin: 0 0.3125rem;
  }

  .content {
    padding: var(--content-padding);
  }

  .description {
    display: inline-flex;
  }

  .tip {
    font-size: 0.875rem;
    line-height: 1.125rem;
    display: flex;
    gap: 0.625rem;
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
        <span class="row" style="margin-left: 0.875rem">
          {#if editing}
            <Input.Text
              dataCy="budget-input"
              bind:value={budget}
              validation={$budgetValidation}
              validationStyle={'position: absolute; top: 1.875rem;'}
              showLeftItem
              autofocus
              style={'width: 7.8rem'}>
              <div
                slot="left"
                style="position: absolute; top: 0.0625rem; left: 0.75rem;">
                <Dai />
              </div>
            </Input.Text>
          {:else}
            <p class="typo-text-bold">
              <Dai>{poolData.amountPerBlock}</Dai>
            </p>
          {/if}
          <span style="margin-left: 0.4375rem;"> per month</span>
        </span>
        <!-- svelte-ignore a11y-missing-attribute -->
        <a
          hidden={editing}
          class="typo-link"
          disabled={ongoingSupportUpdate}
          on:click={enterEditMode}
          style="margin-left: 0.75rem;">
          Edit
        </a>
      </div>
      <div class="row">
        <p>Remaining</p>
        <p class="typo-text-bold row" style="margin-left: 0.75rem">
          <Dai>{poolData.balance}</Dai>
        </p>
        {#if !ongoingWithdraw && !ongoingTopUp}
          <Button
            disabled={poolData.balance === 0}
            dataCy="drain-pool-button"
            variant="transparent"
            on:click={openWithdrawModal}
            style="margin-left: 0.75rem">
            Withdraw
          </Button>
        {/if}
        {#if !ongoingTopUp}
          <Button
            dataCy="top-up-pool-button"
            variant="vanilla"
            on:click={openTopUp}
            style="margin-left: 0.75rem">
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
              style="margin-left: 0.3125rem;"
              on:click={enterEditMode}>
              Edit
            </a>
          </p>
        {:else}
          <div style="display: flex; align-items: center">
            <strong style="margin-left: 0">
              <Dai>{poolData.amountPerBlock}</Dai></strong>
            per month will go to each of the
            <strong>{poolData.receivers.size} </strong>
            receivers you're supporting.

            <!-- svelte-ignore a11y-missing-attribute -->
            <a
              hidden={editing}
              class="typo-link"
              disabled={ongoingSupportUpdate}
              style="margin-left: 0.3125rem;"
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
        disabled={!thereAreChanges || $budgetValidation.status !== ValidationStatus.Success}
        dataCy="confirm-button"
        onClick={onConfirmInWallet}
        errorLabel="Failed to save support settings">
        Confirm in your wallet
      </TxButton>
    </div>
  {/if}
</Remote>
