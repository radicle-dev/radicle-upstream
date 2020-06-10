<script>
  import { pop } from "svelte-spa-router";

  import {
    RegistrationFlowState,
    registerMember,
    registerMemberTransaction,
    memberHandleValidationStore,
  } from "../../src/org.ts";
  import { session } from "../../src/session.ts";
  import { formatPayer } from "../../src/transaction.ts";
  import { ValidationStatus } from "../../src/validation.ts";

  import { Input, Text } from "../../DesignSystem/Primitive";
  import {
    NavigationButtons,
    StepModalLayout,
    Transaction,
  } from "../../DesignSystem/Component";

  export let params = null;
  const orgId = params.id;

  let state = RegistrationFlowState.Preparation;
  let payer,
    subject,
    transaction,
    userHandle,
    validating = false;
  const validation = memberHandleValidationStore();

  const transactionFee = $session.data.transactionCosts.minimumFee;

  const next = () => {
    switch (state) {
      case RegistrationFlowState.Preparation:
        if ($validation.status === ValidationStatus.Success) {
          transaction = registerMemberTransaction(
            orgId,
            userHandle,
            transactionFee
          );
          payer = formatPayer($session.data.identity);
          state = RegistrationFlowState.Confirmation;
        }
        break;
      case RegistrationFlowState.Confirmation:
        registerMember(orgId, userHandle);
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
    if (validating) validation.validate(userHandle);
  }

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
      dataCy="input" />
  {:else if state === RegistrationFlowState.Confirmation}
    <div style="width: 100%;">
      <Transaction
        {transaction}
        {subject}
        {payer}
        transactionCosts={$session.data.transactionCosts} />
    </div>
  {/if}
  <NavigationButtons
    style="margin-top: 32px;"
    {disableSubmit}
    on:submit={next}
    on:cancel={cancel} />
</StepModalLayout>
