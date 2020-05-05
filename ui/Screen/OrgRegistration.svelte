<script>
  import { pop } from "svelte-spa-router";

  import {
    RegistrationFlowState,
    getTransaction,
    validationStore,
    getSubject,
    getPayer,
    register,
    generateAvatar
  } from "../src/org.ts";
  import { session, fetch as fetchSession } from "../src/session.ts";
  import { Status } from "../src/remote.ts";

  import { showNotification } from "../store/notification.js";

  import { ValidationStatus } from "../src/validation.ts";

  import {
    ModalLayout,
    NavigationButtons,
    StepCounter,
    Transaction
  } from "../DesignSystem/Component";
  import { Avatar, Input, Text, Title } from "../DesignSystem/Primitive";

  let orgName, transaction, subject, avatarFallback;
  let state = RegistrationFlowState.NameSelection;

  // Create a new validation store
  let validating = false;
  const validation = validationStore();

  const next = () => {
    switch (state) {
      case RegistrationFlowState.NameSelection:
        if ($validation.status === ValidationStatus.Success) {
          transaction = getTransaction(orgName);
          subject = getSubject(orgName);
          avatarFallback = generateAvatar(orgName);
          state = RegistrationFlowState.TransactionConfirmation;
        }
        break;
      case RegistrationFlowState.TransactionConfirmation:
        registerOrg();
    }
  };

  const registerOrg = async () => {
    try {
      await register(orgName);
      await fetchSession();
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
  }

  $: submitLabel =
    state === RegistrationFlowState.TransactionConfirmation
      ? "Submit transaction"
      : "Next";
  $: disableSubmit = $validation.status !== ValidationStatus.Success;

  $: payer =
    $session.status === Status.Success && getPayer($session.data.identity);
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
        style="width: 100%;"
        showSuccessCheck
        validation={$validation}>
        <div slot="avatar">
          <Avatar {avatarFallback} size="small" variant="square" />
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
