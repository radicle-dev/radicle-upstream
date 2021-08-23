<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import Button from "ui/DesignSystem/Button.svelte";
  import Dai from "ui/DesignSystem/Dai.svelte";
  import Modal from "ui/DesignSystem/Modal.svelte";
  import TextInput from "ui/DesignSystem/TextInput.svelte";

  import { amountStore, balanceValidationStore } from "ui/src/funding/pool";
  import { ValidationStatus } from "ui/src/validation";

  import Big from "big.js";

  export let amount = "";
  export let onBack: [string, () => void];
  export let balance: Big = Big(0);
  export let disabled = true;

  let validating = false;
  $: validation = balanceValidationStore(balance);
  $: amountStore.set(amount);
  $: {
    if ($amountStore && $amountStore.length > 0) {
      validating = true;
    }
    if (validating) {
      validation.validate($amountStore);
    }
  }

  $: disabled = $validation.status !== ValidationStatus.Success;
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
  }
</style>

<Modal emoji="💸" title="Top up your account">
  <svelte:fragment slot="description">
    You can top up a couple of weeks worth of support or just enough for this
    week.
  </svelte:fragment>

  <div class="wrapper">
    <TextInput
      dataCy="modal-amount-input"
      bind:value={amount}
      validation={$validation}
      showLeftItem
      autofocus
      style={"width: 125px; margin-top: 1.5rem"}>
      <div slot="left" style="position: absolute; top: 1px; left: 12px;">
        <Dai />
      </div>
    </TextInput>
  </div>
  <svelte:fragment slot="buttons">
    <Button variant="transparent" dataCy="cancel" on:click={onBack[1]}>
      {onBack[0]}
    </Button>

    <!-- Continue button provided by the parent view !-->
    <slot />
  </svelte:fragment>
</Modal>
