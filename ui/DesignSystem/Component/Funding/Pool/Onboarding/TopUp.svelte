<script lang="typescript">
  import { Button, Input } from "../../../../Primitive";
  import { Dai, Illustration } from "../../../../Component";

  import { wallet } from "../../../../../src/wallet";
  import { Variant as IllustrationVariant } from "../../../../../src/illustration";
  import {
    amountStore,
    topUpAmountValidationStore,
  } from "../../../../../src/funding/pool";
  import { ValidationStatus } from "../../../../../src/validation";

  export let topUp = 0;
  export let onBack: () => void;
  export let onContinue: () => void;

  $: accountBalance = $wallet.connected.account.balance * 1 + 3;

  let validating = false;
  $: validation = topUpAmountValidationStore(accountBalance);
  $: amountStore.set(topUp ? topUp.toString() : "");
  $: {
    if ($amountStore && $amountStore.length > 0) validating = true;
    if (validating) validation.validate($amountStore);
  }

  $: disabled = $validation.status !== ValidationStatus.Success;
</script>

<style>
  h1,
  p {
    padding: 0 var(--content-padding);
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: calc(var(--content-padding) / 2);
  }
</style>

<Illustration variant={IllustrationVariant.Money} />
<h1>Top up your account</h1>
<p>
  You can top up a couple of months worth of support or just enough for this
  month.
</p>
<Input.Text
  dataCy="modal-amount-input"
  bind:value={topUp}
  validation={$validation}
  showLeftItem
  autofocus
  style={'width: 125px'}>
  <div slot="left" style="position: absolute; top: 1px; left: 12px;">
    <Dai />
  </div>
</Input.Text>
<div class="submit">
  <Button
    variant="transparent"
    dataCy="cancel"
    on:click={onBack}
    style="margin-right: 1rem">
    Back
  </Button>

  <Button dataCy="confirm-button" {disabled} on:click={onContinue}>
    Continue
  </Button>
</div>
