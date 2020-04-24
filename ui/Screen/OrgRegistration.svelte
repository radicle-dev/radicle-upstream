<script>
  import { pop } from "svelte-spa-router";

  import {
    RegistrationFlowState,
    createValidationStore,
    ValidationStatus,
    transaction,
    subject,
    payer,
    org
  } from "../src/org.ts";

  import {
    ModalLayout,
    NavigationButtons,
    StepCounter,
    Transaction
  } from "../DesignSystem/Component";
  import { Input, Text, Title } from "../DesignSystem/Primitive";

  let orgName;
  let state = RegistrationFlowState.NameSelection;

  const validation = createValidationStore();

  const next = () => {
    switch (state) {
      case RegistrationFlowState.NameSelection:
        validation.validate(orgName);
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
    if ($validation.status !== ValidationStatus.NotStarted)
      validation.validate(orgName);
  }
  $: valid =
    $validation.status === ValidationStatus.NotStarted ||
    $validation.status === ValidationStatus.Success;

  $: validationMessage =
    $validation.status === ValidationStatus.Error && $validation.message;

  $: subject.name = orgName;
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
      <Input.Text
        placeholder="Org name (e.g. Flowerpot)"
        variant="avatar"
        bind:value={orgName}
        imageUrl={org.avatar.imageUrl}
        style="--focus-outline-color: var(--color-primary); width: 100%;"
        {valid}
        {validationMessage} />
    {:else if state === RegistrationFlowState.TransactionConfirmation}
      <div style="width: 100%;">
        <Transaction {transaction} {subject} {payer} />
      </div>
    {/if}
    <NavigationButtons
      style="margin-top: 32px;"
      nextStepTitle={nextButtonTitle(state)}
      onCancel={cancel}
      onNextStep={next} />
  </div>
</ModalLayout>
