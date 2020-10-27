<script lang="typescript">
  import { Box, Icon, Input } from "../../../../Primitive";

  import {
    amountStore,
    monthlyContributionValidationStore,
  } from "../../../../../src/funding/pool";
  import { ValidationStatus } from "../../../../../src/validation";
  import TxButton from "../../../TxButton.svelte";

  export let style = "";
  // The current set monthly contribution value.
  export let currentValue: string = "";
  // Whether there is already an ongoing tx setting the monthly contribution.
  export let ongoing = false;
  // The action to run on save
  export let onSave: (value: string) => Promise<void>;

  let monthlyContribution = currentValue;
  let validatingAmount = false;
  $: amountValidation = monthlyContributionValidationStore();
  $: amountStore.set(monthlyContribution ? monthlyContribution.toString() : "");
  $: {
    if ($amountStore && $amountStore.length > 0) validatingAmount = true;
    if (validatingAmount) amountValidation.validate($amountStore);
  }

  $: saveMonthlyContributionEnabled =
    $amountValidation &&
    $amountValidation.status === ValidationStatus.Success &&
    monthlyContribution.valueOf() !== currentValue.valueOf();

  $: done = currentValue > 0;
</script>

<style>
  h2,
  p,
  .row {
    margin-top: 1rem;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .row * + * {
    margin-left: 10px;
  }
</style>

<Box {style} {done}>
  <h2>Budget</h2>
  <p>
    Set your monthly budget for outgoing support. This amount will flow to your
    receivers in real time.
  </p>

  <div class="row">
    <Input.Text
      disabled={false}
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
    {#if saveMonthlyContributionEnabled}
      <TxButton
        title="✓"
        disabled={ongoing}
        onClick={() => onSave(monthlyContribution)}
        style="margin-left: 7px"
        variant={'secondary'} />
    {/if}
  </div>
</Box>
