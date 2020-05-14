<script>
  import { pop } from "svelte-spa-router";

  import { getAvatar, Usage } from "../src/avatar.ts";
  import * as notification from "../src/notification.ts";
  import {
    RegistrationFlowState,
    orgIdValidationStore,
    register,
  } from "../src/org.ts";
  import { session, fetch as fetchSession } from "../src/session.ts";
  import { formatPayer, MessageType } from "../src/transaction.ts";
  import { ValidationStatus } from "../src/validation.ts";

  import {
    NavigationButtons,
    StepModalLayout,
    Transaction,
  } from "../DesignSystem/Component";
  import { Avatar, Input, Text } from "../DesignSystem/Primitive";

  let orgId, transaction, subject, payer, avatarFallback, showAvatar;
  let state = RegistrationFlowState.NameSelection;

  // Create a new validation store
  let validating = false;
  const validation = orgIdValidationStore();

  const next = () => {
    switch (state) {
      case RegistrationFlowState.NameSelection:
        if ($validation.status === ValidationStatus.Success) {
          transaction = {
            messages: [
              {
                type: MessageType.OrgRegistration,
                id: orgId,
              },
            ],
          };
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
      await register(orgId);
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

  const updateAvatar = async (id) => {
    if (!id || id.length < 1) {
      showAvatar = false;
      return;
    }

    avatarFallback = await getAvatar(Usage.Org, id);

    // check orgId in case input was cleared during the promise
    showAvatar = orgId.length && !!avatarFallback;
  };

  $: updateAvatar(orgId);
  $: submitLabel =
    state === RegistrationFlowState.TransactionConfirmation
      ? "Submit transaction"
      : "Next";

  $: {
    // Start validating once the user enters something for the first time
    if (orgId && orgId.length > 0) validating = true;
    if (validating) validation.updateInput(orgId);
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
      bind:value={orgId}
      style="width: 100%;"
      showSuccessCheck
      {showAvatar}
      validation={$validation}>
      <div slot="avatar">
        <Avatar
          {avatarFallback}
          size="small"
          variant="square"
          dataCy="avatar" />
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
