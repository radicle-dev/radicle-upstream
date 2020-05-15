<script>
  import { pop } from "svelte-spa-router";

  import { getAvatar, Usage } from "../../src/avatar.ts";
  import { session } from "../../src/session.ts";
  import {
    RegistrationFlowState,
    registerMemberTransaction,
    memberHandleValidationStore,
  } from "../../src/org.ts";
  import { formatPayer } from "../../src/transaction.ts";
  import { ValidationStatus } from "../../src/validation.ts";

  import { Avatar, Input, Text } from "../../DesignSystem/Primitive";
  import {
    NavigationButtons,
    StepModalLayout,
    Transaction,
  } from "../../DesignSystem/Component";

  let state = RegistrationFlowState.Preparation;
  let userHandle,
    avatarFallback,
    showAvatar,
    transaction,
    subject,
    payer,
    validating = false;
  const validation = memberHandleValidationStore();

  const next = () => {
    switch (state) {
      case RegistrationFlowState.Preparation:
        if ($validation.status === ValidationStatus.Success) {
          transaction = registerMemberTransaction("monadic", userHandle);
          payer = formatPayer($session.data.identity);
          state = RegistrationFlowState.Confirmation;
        }
        break;
      case RegistrationFlowState.Confirmation:
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

  $: {
    if (userHandle && userHandle.length > 0) validating = true;
    if (validating) validation.updateInput(userHandle);
  }

  // TODO(sos): replace with user avatar fetch
  const updateAvatar = async (handle) => {
    if (!handle || handle.length < 1) {
      showAvatar = false;
      return;
    }

    avatarFallback = await getAvatar(Usage.Identity, handle);

    // check userHandle in case input was cleared during the promise
    showAvatar = userHandle.length && !!avatarFallback;
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
  {#if state === RegistrationFlowState.Preparation}
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
      dataCy="input">
      <div slot="avatar">
        <Avatar
          {avatarFallback}
          size="small"
          variant="circle"
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
    {disableSubmit}
    on:submit={next}
    on:cancel={cancel} />
</StepModalLayout>
