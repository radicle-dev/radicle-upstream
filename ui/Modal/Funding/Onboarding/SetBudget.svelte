<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Button, Dai, Modal, TextInput } from "ui/DesignSystem";

  import {
    budgetStore,
    weeklyBudgetValidationStore,
  } from "ui/src/funding/pool";
  import { ValidationStatus } from "../../../src/validation";

  export let budget = "";
  export let onCancel: () => void;
  export let onContinue: () => void;

  let validating = false;
  $: validation = weeklyBudgetValidationStore();
  $: budgetStore.set(budget);
  $: {
    if ($budgetStore && $budgetStore.length > 0) {
      validating = true;
    }
    if (validating) {
      validation.validate($budgetStore);
    }
  }

  let disabled = true;
  $: disabled = $validation.status !== ValidationStatus.Success;

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter" && !disabled) {
      onContinue();
    }
  };
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
  }
</style>

<svelte:window on:keydown={onKeydown} />
<Modal emoji="💸" title="Set a weekly budget">
  <svelte:fragment slot="description">
    Set your weekly budget for outgoing support. This amount will flow to your
    receivers in real time.
  </svelte:fragment>

  <div class="wrapper">
    <TextInput
      dataCy="modal-amount-input"
      bind:value={budget}
      validation={$validation}
      showLeftItem
      autofocus
      style={"width: 125px; padding: 0; margin-top: 1.5rem;"}>
      <div slot="left" style="position: absolute; top: 1px; left: 12px;">
        <Dai />
      </div>
    </TextInput>
  </div>

  <svelte:fragment slot="buttons">
    <Button variant="transparent" dataCy="cancel" on:click={onCancel}>
      Cancel
    </Button>

    <Button dataCy="confirm-button" {disabled} on:click={onContinue}>
      Continue
    </Button>
  </svelte:fragment>
</Modal>
