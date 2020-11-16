<script lang="ts">
  import { Button, Input } from "../../../../Primitive";
  import { Dai, Remote, TxButton } from "../../../../Component";

  import Receivers from "../Receivers.svelte";

  import * as modal from "../../../../../src/modal";
  import * as path from "../../../../../src/path";
  import * as remote from "../../../../../src/remote";
  import * as _pool from "../../../../../src/funding/pool";
  import * as transaction from "../../../../../src/transaction";

  import {
    amountStore,
    monthlyContributionValidationStore,
  } from "../../../../../src/funding/pool";
  import { ValidationStatus } from "../../../../../src/validation";

  export let pool: _pool.Pool;

  $: _pool.store.set(pool);

  let ongoingTopUp = false;
  let ongoingWithdraw = false;
  let ongoingMonthlyContributionUpdate = false;
  let ongoingBeneficiariesUpdate = false;

  transaction.store.subscribe(_ => {
    ongoingTopUp = transaction.ongoing(transaction.TxKind.TopUp);
    ongoingWithdraw = transaction.ongoing(transaction.TxKind.Withdraw);
    ongoingMonthlyContributionUpdate = transaction.ongoing(
      transaction.TxKind.UpdateMonthlyContribution
    );
    ongoingBeneficiariesUpdate = transaction.ongoing(
      transaction.TxKind.UpdateBeneficiaries
    );
  });

  // Editing values
  let budget = "";
  let receivers: string[] = [];
  let changeset: _pool.Changeset;

  let validatingBudget = false;
  $: budgetValidation = monthlyContributionValidationStore();
  $: amountStore.set(budget ? budget.toString() : "");
  $: {
    if ($amountStore && $amountStore.length > 0) validatingBudget = true;
    if (validatingBudget) budgetValidation.validate($amountStore);
  }

  let data: _pool.PoolData;
  pool.data.subscribe(store => {
    if (store.status === remote.Status.Success) {
      budget = store.data.amountPerBlock;
      receivers = store.data.receiverAddresses;
      data = store.data;
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
    receivers.sort() !== data.receiverAddresses.sort();

  function leaveEditMode(): void {
    editing = false;
    budget = data.amountPerBlock;
    receivers = data.receiverAddresses;
  }

  function onConfirmInWallet(): Promise<void> {
    console.log("onConfirmInWallet");
    console.log(`Receivers: ${receivers}`);
    console.log(`changeset: ${JSON.stringify(changeset)}`);
    const changesetz = new Map(
      receivers.map(r => [r, _pool.AddressStatus.Added])
    );

    return pool.updateSettings(budget, changesetz).then(_ => leaveEditMode());
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
    padding: var(--content-padding) 0px 0px 0;
    border: 1px solid #ebeff3;
    box-sizing: border-box;
    border-radius: 8px;
  }
  h3,
  p {
    color: var(--color-foreground-level-6);
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--content-padding);
    padding-top: 0px;
    border-bottom: 1px solid #ebeff3;
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

  .tip {
    font-size: 14px;
    line-height: 18px;
    display: flex;
    align-items: center;
    text-align: center;
    margin-top: calc(1.5 * var(--content-padding));
    color: var(--color-foreground-level-5);
  }
  .description {
    display: flex;
    align-items: center;
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
          disabled={ongoingMonthlyContributionUpdate}
          style="margin-left: 12px;"
          on:click={enterEditMode}>
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
      <p class="description">
        <strong style="margin-left: 0px"><Dai>
            {poolData.amountPerBlock}
          </Dai></strong> per month will go to each of the <strong>{poolData.receiverAddresses.length}
        </strong> receivers you're supporting.
        <!-- svelte-ignore a11y-missing-attribute -->
        <a
          hidden={editing}
          class="typo-link"
          disabled={ongoingMonthlyContributionUpdate}
          style="margin-left: 12px;"
          on:click={enterEditMode}>
          Edit
        </a>
      </p>

      <Receivers
        {editing}
        bind:receivers
        bind:changeset
        updating={ongoingBeneficiariesUpdate} />

      <div class="tip">
        ⓘ To stop or pause your generosity, set the monthly contribution to 0.
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
