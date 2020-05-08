<script>
  import { pop } from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import {
    RegistrationFlowState,
    orgNameValidationStore,
    register,
    generateAvatar,
  } from "../src/org.ts";
  import { session, fetch as fetchSession } from "../src/session.ts";
  import {
    formatPayer,
    formatSubject,
    MessageType,
  } from "../src/transaction.ts";
  import { ValidationStatus } from "../src/validation.ts";

  import {
    NavigationButtons,
    StepModalLayout,
    Transaction,
  } from "../DesignSystem/Component";
  import { Avatar, Input, Text } from "../DesignSystem/Primitive";

  let orgName, transaction, subject, payer, avatarFallback;
  let state = RegistrationFlowState.NameSelection;

  // Create a new validation store
  let validating = false;
  const validation = orgNameValidationStore();

  const next = () => {
    switch (state) {
      case RegistrationFlowState.NameSelection:
        if ($validation.status === ValidationStatus.Success) {
          transaction = {
            messages: [
              {
                type: MessageType.OrgRegistration,
                orgId: orgName,
              },
            ],
          };
          avatarFallback = generateAvatar();
          subject = formatSubject(
            $session.data.identity,
            transaction.messages[0]
          );
          payer = formatPayer($session.data.identity);
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
      notification.error(`Could not register org: ${error.message}`);
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

  $: submitLabel =
    state === RegistrationFlowState.TransactionConfirmation
      ? "Submit transaction"
      : "Next";

  $: {
    // Start validating once the user enters something for the first time
    if (orgName && orgName.length > 0) validating = true;
    if (validating) validation.updateInput(orgName);
  }

  $: disableSubmit = $validation.status !== ValidationStatus.Success;
</script>

<StepModalLayout
  dataCy="org-reg-modal"
  selectedStep={state + 1}
  steps={['Prepare', 'Submit']}>
  <div slot="title">Register an org</div>
  {#if state === RegistrationFlowState.NameSelection}
    <Text style="color: var(--color-foreground-level-5); margin-bottom: 24px;">
      Registering an org allows you to give others in your org the right to sign
      transactions, like adding other members or adding projects.
    </Text>
    <Input.Text
      dataCy="name-input"
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
</StepModalLayout>
