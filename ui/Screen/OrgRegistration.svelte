<script>
  import { pop } from "svelte-spa-router";

  import {
    RegistrationFlowState,
    transaction,
    subject,
    payer
  } from "../src/org.ts";

  import {
    ModalLayout,
    NavigationButtons,
    StepCounter,
    Transaction,
    ValidatedInput
  } from "../DesignSystem/Component";
  import { Text, Title } from "../DesignSystem/Primitive";

  let state = RegistrationFlowState.NameSelection;

  const next = () => {
    switch (state) {
      case RegistrationFlowState.NameSelection:
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

  let identifier;
  $: subject.name = identifier;
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
      <ValidatedInput
        inputPlaceholder="Org name (e.g. Flowerpot)"
        entity="Org name"
        bind:identifier />
    {:else if state === RegistrationFlowState.TransactionConfirmation}
      <div style="width: 100%; margin-bottom: 32px;">
        <Transaction {transaction} {subject} {payer} />
      </div>
    {/if}
    <NavigationButtons
      nextStepTitle={nextButtonTitle(state)}
      onCancel={cancel}
      onNextStep={next} />
  </div>
</ModalLayout>
