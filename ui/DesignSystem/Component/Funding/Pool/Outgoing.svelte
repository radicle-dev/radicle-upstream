<script lang="ts">
  import { Button, Icon, Input } from "../../../Primitive";
  import { Remote, TxButton } from "../../../Component";

  import * as modal from "../../../../src/modal";
  import * as path from "../../../../src/path";
  import * as remote from "../../../../src/remote";
  import * as _pool from "../../../../src/funding/pool";
  import * as transaction from "../../../../src/transaction";

  import {
    amountStore,
    monthlyContributionValidationStore,
    membersStore,
    membersValidationStore,
  } from "../../../../src/funding/pool";

  import { ValidationStatus } from "../../../../src/validation";

  export let pool: _pool.Pool;

  // The loaded PoolData, updated at on `pool.data.subscribe`.
  let data: _pool.PoolData | undefined;

  let ongoingTopUp = false;
  let ongoingMonthlyContributionUpdate = false;
  let ongoingBeneficiariesUpdate = false;

  transaction.store.subscribe(_ => {
    ongoingTopUp = transaction.ongoing(transaction.TxKind.TopUp);
    ongoingMonthlyContributionUpdate = transaction.ongoing(
      transaction.TxKind.UpdateMonthlyContribution
    );
    ongoingBeneficiariesUpdate = transaction.ongoing(
      transaction.TxKind.UpdateBeneficiaries
    );
  });

  pool.data.subscribe(store => {
    if (store.status === remote.Status.Success) {
      const newData = store.data;
      data = newData;
      monthlyContribution = newData.amountPerBlock;
      members = newData.receiverAddresses.join(",");
    }
  });

  let monthlyContribution = "";
  let validatingAmount = false;
  $: amountValidation = monthlyContributionValidationStore();
  $: amountStore.set(monthlyContribution ? monthlyContribution.toString() : "");
  $: {
    if ($amountStore && $amountStore.length > 0) validatingAmount = true;
    if (validatingAmount) amountValidation.validate($amountStore);
  }

  // Necessary type to comply with Textarea.bind:value type.
  let members: string = "";
  let validatingMembers = false;
  $: membersStore.set(members ? members.toString() : "");
  $: {
    if ($membersStore && $membersStore.length > 0) validatingMembers = true;
    if (validatingMembers) membersValidationStore.validate($membersStore);
  }

  $: saveMonthlyContributionEnabled =
    $amountValidation &&
    $amountValidation.status === ValidationStatus.Success &&
    data &&
    monthlyContribution.valueOf() !== data.amountPerBlock.valueOf();

  $: saveMembersEnabled =
    $membersValidationStore &&
    $membersValidationStore.status === ValidationStatus.Success &&
    data &&
    extractMembers(members).join(",") !== data.receiverAddresses.join(",");

  const openSendModal = () => {
    _pool.store.set(pool);
    modal.toggle(path.poolTopUp());
  };

  // Extract the list of unique members from the provided raw input string.
  function extractMembers(raw: string): string[] {
    return [
      ...new Set(
        raw
          .split(",")
          .map(e => e.trim())
          .filter(e => e.length > 0)
      ),
    ];
  }

  /* On clicks */

  function onSaveMonthlyContribution(): Promise<void> {
    return pool.updateAmountPerBlock(monthlyContribution);
  }

  function onSaveMembers(): Promise<void> {
    if (!data) {
      throw new Error("Something went wrong");
    }
    return pool.updateReceiverAddresses(data, extractMembers(members));
  }
</script>

<style>
  .outgoing-container {
    margin: 2rem 0;
    padding: var(--content-padding) 0px;

    border: 1px solid #ebeff3;
    box-sizing: border-box;
    border-radius: 8px;
  }

  h3,
  p {
    color: #546474;
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

  .row * + * {
    margin-left: 10px;
  }

  strong {
    font-weight: bold;
  }

  .content {
    padding: var(--content-padding);
  }
</style>

<div class="outgoing-container">
  <Remote store={pool.data} let:data={poolData}>
    <header>
      <div class="row">
        <h3>Outgoing support</h3>
        <span class="row">
          <Input.Text
            disabled={ongoingMonthlyContributionUpdate}
            dataCy="modal-amount-input"
            placeholder="Enter the amount"
            bind:value={monthlyContribution}
            showLeftItem
            validation={$amountValidation}
            style="max-width: 150px; margin-left: 10px;">
            <div slot="left" style="position: absolute; top: 9px; left: 10px;">
              <Icon.CurrencyDAI style="fill: var(--color-foreground-level-6)" />
            </div>
          </Input.Text>
          <p>per month</p>
        </span>
        {#if saveMonthlyContributionEnabled && !ongoingMonthlyContributionUpdate}
          <TxButton
            disabled={!saveMonthlyContributionEnabled}
            title={'Save'}
            onClick={onSaveMonthlyContribution}
            variant={'transparent'}
            errorMessage={e => `Failed to save pool: ${e}`} />
        {/if}
      </div>
      <div class="row">
        <p>Balance</p>
        <h3>{poolData.balance} DAI</h3>
        {#if !ongoingTopUp}
          <Button
            dataCy="top-up-pool-button"
            variant="vanilla"
            on:click={openSendModal}
            style="margin-left: 12px">
            Top up
          </Button>
        {/if}
        <Button
          dataCy="drain-pool-button"
          variant="outline"
          disabled
          style="margin-left: 12px">
          Drain
        </Button>
      </div>
    </header>

    <div class="content">
      <p>
        <strong>{poolData.amountPerBlock} DAI</strong> per month will be taken from
        your <strong>{poolData.balance} DAI</strong> that you’ve added to this contract,
        which means that <strong>{poolData.amountPerBlock / poolData.receiverAddresses.length}
          DAI</strong> per month will be evenly spread between the <strong>{poolData.receiverAddresses.length}</strong>
        people you’re supporting. To keep the support going, top up by <strong>October
          14th</strong> (TODO).
      </p>

      <div style="margin-top: var(--content-padding)">
        <Input.Textarea
          disabled={ongoingBeneficiariesUpdate}
          validation={$membersValidationStore}
          style="min-width: 400px;"
          bind:value={members}
          placeholder="Enter a list of comma-separated addresses here" />

        {#if saveMembersEnabled && !ongoingBeneficiariesUpdate}
          <TxButton
            disabled={!saveMembersEnabled}
            title={'Save'}
            onClick={onSaveMembers}
            variant={'outline'}
            errorMessage={e => `Failed to save pool: ${e}`} />
        {/if}
      </div>
    </div>
  </Remote>
</div>
