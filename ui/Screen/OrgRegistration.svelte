<script>
  import { pop } from "svelte-spa-router";

  import { getAvatar, Usage } from "../src/avatar.ts";
  import { idValidationStore } from "../src/id.ts";
  import * as notification from "../src/notification.ts";
  import { RegistrationFlowState, register } from "../src/org.ts";
  import { session, fetch as fetchSession } from "../src/session.ts";
  import { formatPayer, MessageType } from "../src/transaction.ts";
  import { ValidationStatus } from "../src/validation.ts";

  import {
    NavigationButtons,
    StepModalLayout,
    Transaction,
  } from "../DesignSystem/Component";
  import { Avatar, Input, Text } from "../DesignSystem/Primitive";

  let avatarFallback, orgId, payer, showAvatar, subject, transaction;
  let state = RegistrationFlowState.Preparation;

  const transactionFee = $session.data.minimumTransactionFee;

  const next = () => {
    switch (state) {
      case RegistrationFlowState.Preparation:
        if ($validation.status === ValidationStatus.Success) {
          transaction = {
            fee: transactionFee,
            messages: [
              {
                type: MessageType.OrgRegistration,
                id: orgId,
              },
            ],
          };
          payer = formatPayer($session.data.identity);
          state = RegistrationFlowState.Confirmation;
        }
        break;
      case RegistrationFlowState.Confirmation:
        registerOrg();
    }
  };

  const registerOrg = async () => {
    try {
      await register(orgId, transactionFee);
      await fetchSession();
    } catch (error) {
      notification.error(`Could not register org: ${error.message}`);
    } finally {
      pop();
    }
  };

  const cancel = () => {
    switch (state) {
      case RegistrationFlowState.Preparation:
        pop();
        break;
      case RegistrationFlowState.Confirmation:
        state = RegistrationFlowState.Preparation;
    }
  };

  const updateAvatar = async id => {
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
    state === RegistrationFlowState.Confirmation
      ? "Submit transaction"
      : "Next";

  // Create a new validation store
  const validation = idValidationStore();
  let userStartedInputting = false;
  $: {
    // Start validating once the user enters something for the first time
    if (orgId && orgId.length > 0) userStartedInputting = true;
    if (userStartedInputting) validation.validate(orgId);
  }

  $: disableSubmit = $validation.status !== ValidationStatus.Success;
</script>

<StepModalLayout
  dataCy="org-reg-modal"
  selectedStep={state + 1}
  steps={['Preparation', 'Submit']}>
  <div slot="title">Register an org</div>
  {#if state === RegistrationFlowState.Preparation}
    <Text style="color: var(--color-foreground-level-5); margin-bottom: 24px;">
      Registering an org allows you to give others in your org the right to sign
      transactions, like adding other members or adding projects.
    </Text>
    <Input.Text
      dataCy="input"
      placeholder="Org name (e.g. flowerpot)"
      bind:value={orgId}
      style="width: 100%;"
      showSuccessCheck
      showLeftItem={showAvatar}
      validation={$validation}>
      <div slot="left">
        <Avatar
          {avatarFallback}
          size="small"
          variant="square"
          dataCy="avatar" />
      </div>
    </Input.Text>
  {:else if state === RegistrationFlowState.Confirmation}
    <div style="width: 100%;">
      <Transaction
        {transaction}
        {subject}
        {payer}
        transactionDeposits={$session.data.transactionDeposits} />
    </div>
  {/if}
  <NavigationButtons
    style="margin-top: 32px;"
    {submitLabel}
    {disableSubmit}
    on:cancel={cancel}
    on:submit={next} />
</StepModalLayout>
