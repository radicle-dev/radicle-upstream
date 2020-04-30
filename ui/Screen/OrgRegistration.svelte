<script>
  import { pop } from "svelte-spa-router";

  import {
    RegistrationFlowState,
    transaction,
    nameConstraints,
    subject,
    payer,
    org,
    register
  } from "../src/org.ts";
  import { showNotification } from "../store/notification.js";

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
  import { Avatar, Input, Text, Title } from "../DesignSystem/Primitive";

  let orgName;
  let state = RegistrationFlowState.NameSelection;

  const validation = createValidationStore(nameConstraints);
  let validating = false;

  const next = () => {
    switch (state) {
      case RegistrationFlowState.NameSelection:
        if ($validation.status === ValidationStatus.Success)
          state = RegistrationFlowState.TransactionConfirmation;
        break;
      case RegistrationFlowState.TransactionConfirmation:
        registerOrg();
    }
  };

  const registerOrg = async () => {
    try {
      await register(orgName);
    } catch (error) {
      showNotification({
        text: `Could not register org: ${error.message}`,
        level: "error"
      });
    } finally {
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

  $: {
    // Start validating once the user enters something for the first time
    if (orgName && orgName.length > 0) validating = true;
    if (validating) validation.updateInput(orgName);

    subject.name = orgName;
  }

  $: submitLabel =
    state === RegistrationFlowState.TransactionConfirmation
      ? "Submit transaction"
      : "Next";

  $: disableSubmit = $validation.status !== ValidationStatus.Success;
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
        bind:value={orgName}
        style="--focus-outline-color: var(--color-primary); width: 100%;"
        showSuccessCheck
        validation={$validation}>
        <div slot="avatar">
          <Avatar
            imageUrl={org.avatar.imageUrl}
            size="small"
            variant="square" />
        </div>
      </Input.Text>
    {:else if state === RegistrationFlowState.TransactionConfirmation}
      <div style="width: 100%;">
        <Transaction {transaction} {subject} {payer} />
      </div>
    {/if}
    <NavigationButtons
      style="margin-top: 32px;"
      {submitLabel}
      {disableSubmit}
      on:cancel={cancel}
      on:submit={next} />
  </div>
</ModalLayout>
