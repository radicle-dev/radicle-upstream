<script>
  import { pop } from "svelte-spa-router";

  import { getAvatar, Usage } from "../src/avatar.ts";
  import { idValidationStore } from "../src/id.ts";
  import * as notification from "../src/notification.ts";
  import { RegistrationFlowState, register } from "../src/org.ts";
  import { session, fetch as fetchSession } from "../src/session.ts";
  import { getPayer, MessageType } from "../src/transaction.ts";
  import { ValidationStatus } from "../src/validation.ts";

  import {
    NavigationButtons,
    ModalLayout,
    Transaction,
  } from "../DesignSystem/Component";
  import { Avatar, Input, Text, Title } from "../DesignSystem/Primitive";

  let avatarFallback, orgId, payer, showAvatar, subject, transaction;
  let state = RegistrationFlowState.Preparation;

  const transactionFee = $session.data.minimumTransactionFee;
  const registrationFee = $session.data.registrationFee.org;

  const next = () => {
    switch (state) {
      case RegistrationFlowState.Preparation:
        if ($validation.status === ValidationStatus.Success) {
          transaction = {
            fee: transactionFee,
            registrationFee: registrationFee,
            messages: [
              {
                type: MessageType.OrgRegistration,
                id: orgId,
              },
            ],
          };
          payer = getPayer(transaction.messages[0], $session.data);
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

<style>
  .wrapper {
    margin: 92px 0 32px 0;
  }
</style>

<ModalLayout dataCy="org-registration-modal">
  <div class="wrapper">
    {#if state === RegistrationFlowState.Preparation}
      <Title variant="big" style="text-align: center;">Org registration</Title>
      <Text
        style="color: var(--color-foreground-level-5); margin: 16px 0 24px 0;">
        Registering an org allows you to give others in your org the right to
        sign transactions, like adding other members or adding projects.
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
