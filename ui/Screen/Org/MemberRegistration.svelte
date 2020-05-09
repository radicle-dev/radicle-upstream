<script>
  import { pop } from "svelte-spa-router";

  import * as avatar from "../../src/avatar.ts";
  import { session } from "../../src/session.ts";
  import {
    RegistrationFlowState,
    registerMemberTransaction,
    memberNameValidationStore,
  } from "../../src/org.ts";
  import { formatPayer, formatSubject } from "../../src/transaction.ts";
  import { ValidationStatus } from "../../src/validation.ts";

  import { Avatar, Input, Text } from "../../DesignSystem/Primitive";
  import {
    NavigationButtons,
    StepModalLayout,
    Transaction,
  } from "../../DesignSystem/Component";

  let state = RegistrationFlowState.NameSelection;
  let userHandle,
    avatarFallback,
    showAvatar,
    transaction,
    subject,
    payer,
    validating = false;
  const validation = memberNameValidationStore();

  const next = () => {
    switch (state) {
      case RegistrationFlowState.NameSelection:
        if ($validation.status === ValidationStatus.Success) {
          transaction = registerMemberTransaction("monadic", userHandle);
          subject = formatSubject(
            $session.data.identity,
            transaction.messages[0]
          );
          payer = formatPayer($session.data.identity);
          state = RegistrationFlowState.TransactionConfirmation;
        }
        break;
      case RegistrationFlowState.TransactionConfirmation:
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
    if (userHandle && userHandle.length > 0) validating = true;
    if (validating) validation.updateInput(userHandle);
  }

  const updateAvatar = async (handle) => {
    if (handle && handle !== "") {
      avatarFallback = await avatar.get(avatar.Usage.Identity, handle);
      showAvatar = !!avatarFallback;
      return;
    }

    showAvatar = false;
  };

  $: updateAvatar(userHandle);
  $: imageUrl = $validation.status === ValidationStatus.Success && "";
  $: disableSubmit = $validation.status !== ValidationStatus.Success;
</script>

<StepModalLayout
  steps={['Prepare', 'Submit']}
  selectedStep={state + 1}
  dataCy="add-member-modal">
  <div slot="title">Register a member</div>
  {#if state === RegistrationFlowState.NameSelection}
    <Text style="color: var(--color-foreground-level-5); margin-bottom: 24px;">
      Registering a member will allow them to sign transactions for your org.
      Only registered users can be added as members.
    </Text>
    <Input.Text
      placeholder="Registered user id"
      bind:value={userHandle}
      style="width: 100%;"
      showSuccessCheck
      validation={$validation}
      {showAvatar}
      dataCy="name-input">
      <div slot="avatar">
        <Avatar
          {avatarFallback}
          size="small"
          variant="circle"
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
    {disableSubmit}
    on:submit={next}
    on:cancel={cancel} />
</StepModalLayout>
