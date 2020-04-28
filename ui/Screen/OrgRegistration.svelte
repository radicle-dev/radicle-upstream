<script>
  import { pop } from "svelte-spa-router";

  import {
    RegistrationFlowState,
    transaction,
    nameConstraints,
    subject,
    payer,
    org
  } from "../src/org.ts";

  import {
    ValidationStatus,
    createValidationStore
  } from "../src/validation.ts";

  import {
    ModalLayout,
    NavigationButtons,
    StepCounter,
    Transaction
  } from "../DesignSystem/Component";
  import { Input, Text, Title } from "../DesignSystem/Primitive";

  let orgName;
  let state = RegistrationFlowState.NameSelection;

  const validation = createValidationStore(nameConstraints);
  let validating = false;

  const next = () => {
    switch (state) {
      case RegistrationFlowState.NameSelection:
        validating = true;
        if ($validation.status === ValidationStatus.Success)
          state = RegistrationFlowState.TransactionConfirmation;
        break;
      case RegistrationFlowState.TransactionConfirmation:
        console.log("submitting transaction");
        pop();
    }
  };

  const cancel = () => {
    switch (state) {
      case RegistrationFlowState.NameSelection:
        pop();
        break;
      case RegistrationFlowState.TransactionConfirmation:
        state = RegistrationFlowState.NameSelection;
    }
  };

  const nextButtonTitle = () => {
    switch (state) {
      case RegistrationFlowState.NameSelection:
        return "Next";
      case RegistrationFlowState.TransactionConfirmation:
        return "Submit transaction";
    }
  };

  $: {
    // Start validating once the user enters something for the first time
    if (orgName && orgName.length > 0) validating = true;

    subject.name = orgName;
    if (validating) validation.updateInput(orgName);
  }
</script>

<style>
  .container {
    margin: 86px 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
</style>

<ModalLayout>
  <div class="container">
    <StepCounter
      selectedStep={state + 1}
      steps={['Prepare', 'Submit']}
      style="margin-bottom: 50px;" />
    <Title variant="big" style="margin-bottom: 16px;">Register an org</Title>
    {#if state === RegistrationFlowState.NameSelection}
      <Text
        style="color: var(--color-foreground-level-5); margin-bottom: 24px;">
        Registering an org allows you to give others in your org the right to
        sign transactions, like adding other members or adding projects.
      </Text>
      <Input.Validated
        placeholder="Org name (e.g. Flowerpot)"
        variant="avatar"
        bind:value={orgName}
        imageUrl={org.avatar.imageUrl}
        style="--focus-outline-color: var(--color-primary); width: 100%;"
        validation={$validation} />
    {:else if state === RegistrationFlowState.TransactionConfirmation}
      <div style="width: 100%;">
        <Transaction {transaction} {subject} {payer} />
      </div>
    {/if}
    <NavigationButtons
      style="margin-top: 32px;"
      submitLabel={nextButtonTitle(state)}
      on:cancel={cancel}
      on:submit={next}
      disableSubmit={$validation.status !== ValidationStatus.Success} />
  </div>
</ModalLayout>
