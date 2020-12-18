<script lang="typescript">
  import { Button, Input } from "../../../../Primitive";
  import { Dai, Illustration } from "../../../../Component";

  import { Variant as IllustrationVariant } from "../../../../../src/illustration";
  import {
    budgetStore,
    monthlyContributionValidationStore,
  } from "../../../../../src/funding/pool";
  import { ValidationStatus } from "../../../../../src/validation";

  export let budget = 0;
  export let onCancel: () => void;
  export let onContinue: () => void;

  let validating = false;
  $: validation = monthlyContributionValidationStore();
  $: budgetStore.set(budget ? budget.toString() : "");
  $: {
    if ($budgetStore && $budgetStore.length > 0) validating = true;
    if (validating) validation.validate($budgetStore);
  }

  $: disabled = $validation.status !== ValidationStatus.Success;
</script>

<style>
  h1,
  p,
  .submit {
    margin-top: 1.5rem;
  }

  h1,
  p {
    padding: 0 2.5rem;
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
  }
</style>

<Illustration variant={IllustrationVariant.Money} />
<h1>Set a monthly budget</h1>
<p>
  Set your monthly budget for outgoing support. This amount will flow to your
  receivers in real time.
</p>
<Input.Text
  dataCy="modal-amount-input"
  bind:value={budget}
  validation={$validation}
  showLeftItem
  autofocus
  style={'width: 125px; padding: 0; margin-top: 1.5rem;'}>
  <div slot="left" style="position: absolute; top: 1px; left: 12px;">
    <Dai />
  </div>
</Input.Text>
<div class="submit">
  <Button
    variant="transparent"
    dataCy="cancel"
    on:click={onCancel}
    style="margin-right: 1rem">
    Cancel
  </Button>

  <Button dataCy="confirm-button" {disabled} on:click={onContinue}>
    Continue
  </Button>
</div>
