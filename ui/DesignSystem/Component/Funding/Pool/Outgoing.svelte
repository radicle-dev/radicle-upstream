<script lang="ts">
  import { Button, Icon, Input } from "../../../Primitive";
  import { Remote, TxButton } from "../../../Component";
  import Add from "./Onboarding/Add.svelte";
  import Budget from "./Onboarding/Budget.svelte";
  import TopUp from "./Onboarding/TopUp.svelte";

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

  $: _pool.store.set(pool);

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

  let onboardingStatus = new _pool.OnboardingStatus();

  pool.data.subscribe(store => {
    if (store.status === remote.Status.Success) {
      const newData = store.data;
      data = newData;
      monthlyContribution = newData.amountPerBlock;
      members = newData.receiverAddresses.join(",");
      onboardingStatus = new _pool.OnboardingStatus(newData);
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

  function onEditMonthlyContribution() {
    modal.toggle(path.updateMonthlyContribution());
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

  strong {
    font-weight: bold;
  }

  .content {
    padding: var(--content-padding);
  }

  .onboarding .steps {
    display: flex;
    justify-content: space-between;
  }

  .onboarding {
    margin-bottom: var(--content-padding);
  }

  .onboarding h3 {
    margin-bottom: 1rem;
  }
</style>

<div class="outgoing-container">
  <Remote store={pool.data} let:data={poolData}>
    <header>
      <div class="row">
        <h3>Outgoing support</h3>
        <span class="row" style="margin-left: 14px">
          <Icon.CurrencyDAI
            style="fill: var(--color-foreground-level-6); padding-top: 3px; width: 20px; height: 20px;" />
          <p>{poolData.amountPerBlock} per month</p>
        </span>
        <Button
          disabled={ongoingMonthlyContributionUpdate}
          style="margin-left: 10px"
          on:click={onEditMonthlyContribution}
          variant={'transparent'}>
          {poolData.amountPerBlock === '0' ? 'Set' : 'Edit'}
        </Button>
      </div>
      <div class="row">
        <p>Balance</p>
        <p class="typo-text-bold row" style="margin-left: 10px">
          <Icon.CurrencyDAI
            style="fill: var(--color-foreground-level-6); padding-top: 3px;" />
          {poolData.balance}
        </p>
        {#if !ongoingTopUp}
          <Button
            dataCy="top-up-pool-button"
            variant="primary"
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
      {#if !onboardingStatus.isComplete()}
        <div class="onboarding">
          <h3>Getting Started</h3>
          <div class="steps">
            <Add done={onboardingStatus.receivers} />
            <Budget
              currentValue={poolData.amountPerBlock}
              ongoing={ongoingMonthlyContributionUpdate}
              onEdit={onEditMonthlyContribution}
              style={'margin-left: 20px'} />
            <TopUp
              style={'margin-left: 20px'}
              balance={poolData.balance}
              ongoing={ongoingTopUp} />
          </div>
        </div>
      {:else}
        <p>
          <strong>{poolData.amountPerBlock} DAI</strong> will be sent from your balance
          over the course of a month. <strong>{poolData.amountPerBlock / poolData.receiverAddresses.length}
            DAI
          </strong> per month will go to each of the <strong>{poolData.receiverAddresses.length}
            receiver{poolData.receiverAddresses.length === 1 ? '' : 's'}
          </strong> you’re supporting.
        </p>
      {/if}

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
