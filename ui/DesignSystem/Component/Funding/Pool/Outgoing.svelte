<script lang="ts">
  import { Remote } from "../../../Component";

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
</script>

<Remote store={pool.data} let:data={poolData}>
  {#if !onboardingStatus.isComplete()}
    <GetStarted {pool} />
  {:else}
    <h2>You are now setup :)</h2>
  {/if}
</Remote>
