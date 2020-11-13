<script lang="ts">
  import { Button, Icon } from "../../../Primitive";
  import { Dai, Remote } from "../../../Component";

  import Receivers from "./Receivers.svelte";
  import Add from "./Onboarding/Add.svelte";
  import Budget from "./Onboarding/Budget.svelte";
  import TopUp from "./Onboarding/TopUp.svelte";
  import GetStarted from "./GetStarted.svelte";

  import * as modal from "../../../../src/modal";
  import * as path from "../../../../src/path";
  import * as remote from "../../../../src/remote";
  import * as _pool from "../../../../src/funding/pool";
  import * as transaction from "../../../../src/transaction";

  import {
    amountStore,
    monthlyContributionValidationStore,
  } from "../../../../src/funding/pool";

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

  let onboardingStatus = new _pool.OnboardingStatus();

  pool.data.subscribe(store => {
    if (store.status === remote.Status.Success) {
      const newData = store.data;
      monthlyContribution = newData.amountPerBlock;
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

  const openSendModal = () => {
    _pool.store.set(pool);
    modal.toggle(path.poolTopUp());
  };

  const openWithdrawModal = () => {
    _pool.store.set(pool);
    modal.toggle(path.poolWithdraw());
  };

  /* On clicks */

  function onEditMonthlyContribution() {
    modal.toggle(path.updateMonthlyContribution());
  }

  function onSaveReceivers(changeset: _pool.Changeset): Promise<void> {
    console.log("onSaveReceivers: ", changeset);
    return pool.updateReceiverAddresses(changeset);
  }
</script>

<style>
  .outgoing-container {
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
</style>

<Remote store={pool.data} let:data={poolData}>
  {#if !onboardingStatus.isComplete()}
    <GetStarted />
  {/if}
</Remote>
