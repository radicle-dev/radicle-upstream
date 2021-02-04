<script lang="typescript">
  import { Button, Emoji, Input } from "../../../Primitive";
  import { Dai } from "../../../Component";

  import {
    amountStore,
    balanceValidationStore,
  } from "../../../../src/funding/pool";
  import { ValidationStatus } from "../../../../src/validation";

  import Big from "big.js";

  export let amount = "";
  export let onBack: [string, () => void];
  export let balance: Big = Big(0);
  export let disabled = true;

  let validating = false;
  $: validation = balanceValidationStore(balance);
  $: amountStore.set(amount);
  $: {
    if ($amountStore && $amountStore.length > 0) validating = true;
    if (validating) validation.validate($amountStore);
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

<Emoji emoji="💸" size="huge" />
<h1>Top up your account</h1>
<p>
  You can top up a couple of weeks worth of support or just enough for this
  week.
</p>
<Input.Text
  dataCy="modal-amount-input"
  bind:value={amount}
  validation={$validation}
  showLeftItem
  autofocus
  style={'width: 125px; margin-top: 1.5rem'}>
  <div slot="left" style="position: absolute; top: 1px; left: 12px;">
    <Dai />
  </div>
</Input.Text>
<div class="submit">
  <Button
    variant="transparent"
    dataCy="cancel"
    on:click={onBack[1]}
    style="margin-right: 1rem">
    {onBack[0]}
  </Button>

  <!-- Continue button provided by the parent view !-->
  <slot />
</div>
