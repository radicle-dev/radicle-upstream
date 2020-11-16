<script lang="ts">
  import { Button } from "../../../../Primitive";
  import { Dai, Remote } from "../../../../Component";

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

  export let pool: _pool.Pool;

  $: _pool.store.set(pool);

  // Flags whether the view is in editing mode.
  // Triggered by the user.
  let editing = false;

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

  let monthlyContribution = "";
  let validatingAmount = false;
  $: amountValidation = monthlyContributionValidationStore();
  $: amountStore.set(monthlyContribution ? monthlyContribution.toString() : "");
  $: {
    if ($amountStore && $amountStore.length > 0) validatingAmount = true;
    if (validatingAmount) amountValidation.validate($amountStore);
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const openTopUp = () => {
    _pool.store.set(pool);
    modal.toggle(path.poolTopUp());
  };

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const openWithdrawModal = () => {
    _pool.store.set(pool);
    modal.toggle(path.poolWithdraw());
  };

  pool.data.subscribe(store => {
    if (store.status === remote.Status.Success) {
      const newData = store.data;
      monthlyContribution = newData.amountPerBlock;
    }
  });

  function enterEditMode(): void {
    editing = true;
  }
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
          <Dai>{poolData.amountPerBlock} per month</Dai>
        </span>
        <!-- svelte-ignore a11y-missing-attribute -->
        <a
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
          class="typo-link"
          disabled={ongoingMonthlyContributionUpdate}
          style="margin-left: 12px;"
          on:click={enterEditMode}>
          Edit
        </a>
      </p>

      <Receivers
        {editing}
        receivers={poolData.receiverAddresses}
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
        on:click={() => (editing = false)}
        style="margin-right: 1rem">
        Discard changes
      </Button>

      <Button
        dataCy="confirm-button"
        on:click={() => console.log('TODO(nuno)')}>
        Confirm in your wallet
      </Button>
    </div>
  {/if}
</Remote>
