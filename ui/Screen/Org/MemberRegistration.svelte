<script>
  import { pop } from "svelte-spa-router";

  import {
    RegistrationFlowState,
    registerMember,
    registerMemberTransaction,
    memberHandleValidationStore,
  } from "../../src/org.ts";
  import { session } from "../../src/session.ts";
  import { getPayer } from "../../src/transaction.ts";
  import { ValidationStatus } from "../../src/validation.ts";

  import { Input } from "../../DesignSystem/Primitive";
  import {
    NavigationButtons,
    ModalLayout,
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
  const validation = memberHandleValidationStore(orgId);

  const transactionFee = $session.data.minimumTransactionFee;
  const registrationFee = $session.data.registrationFee.member;

  const next = () => {
    switch (state) {
      case RegistrationFlowState.Preparation:
        if ($validation.status === ValidationStatus.Success) {
          transaction = registerMemberTransaction(
            orgId,
            userHandle,
            transactionFee,
            registrationFee
          );
          payer = getPayer(transaction.messages[0], $session.data);
          state = RegistrationFlowState.Confirmation;
        }
        break;
      case RegistrationFlowState.Confirmation:
        registerMember(orgId, userHandle, transactionFee);
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

  $: disableSubmit = $validation.status !== ValidationStatus.Success;
</script>

<style>
  .wrapper {
    margin: 92px 0 32px 0;
  }

  h2 {
    text-align: center;
  }

  p {
    color: var(--color-foreground-level-5);
    margin: 16px 0 24px 0;
  }
</style>

<ModalLayout dataCy="add-member-modal">
  <div class="wrapper">
    {#if state === RegistrationFlowState.Preparation}
      <h2>Member registration</h2>
      <p>
        Registering a member will allow them to sign transactions for your org.
        Only registered users can be added as members.
      </p>
      <Input.Text
        placeholder="Registered user id"
        bind:value={userHandle}
        style="width: 100%;"
        showSuccessCheck
        validation={$validation}
        dataCy="input" />
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
  </div>
</ModalLayout>
