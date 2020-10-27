<script lang="typescript">
  import { Box, Button, Icon, Input } from "../../../../Primitive";
  import {
    amountStore,
    monthlyContributionValidationStore,
  } from "../../../../../src/funding/pool";
  import { ValidationStatus } from "../../../../../src/validation";

  export let style = "";
  export let currentValue: string = "";

  let monthlyContribution = "";

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
</script>

<style>
  h2,
  p {
    margin-bottom: 1rem;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  .tip {
    font-size: 14px;
    line-height: 18px;

    display: flex;
    align-items: center;
    text-align: center;

    color: var(--color-foreground-level-5);
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

<Box {style}>
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
      <Button style="margin-left: 7px" variant={'secondary'}>✓</Button>
    {/if}
  </div>
</Box>
