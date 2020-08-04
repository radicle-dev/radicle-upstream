<script>
  import { getContext } from "svelte";
  import { pop } from "svelte-spa-router";

  import { fallback } from "../src/identity.ts";
  import * as notification from "../src/notification.ts";
  import * as session from "../src/session.ts";
  import * as user from "../src/user.ts";

  import { ModalLayout } from "../DesignSystem/Component";

  import PickHandleStep from "./UserRegistration/PickHandleStep.svelte";
  import SubmitRegistrationStep from "./UserRegistration/SubmitRegistrationStep.svelte";

  let { identity } = getContext("session");
  const { minimumTransactionFee } = getContext("session");

  let handle = identity ? identity.metadata.handle : null;
  const id = identity ? identity.id : null;
  identity = identity ? identity : fallback;

  let step = 1;

  const nextStep = () => {
    step += 1;
  };

  const previousStep = () => {
    step -= 1;
  };

  const transactionFee = minimumTransactionFee;

  const registerUser = async () => {
    try {
      await user.register(handle, transactionFee, id);
      await session.fetch();
    } catch (error) {
      notification.error(`Could not register user: ${error}`);
    } finally {
      pop();
    }
  };
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    margin: 92px 0 32px 0;
  }

  .register-user {
    text-align: left;
    width: 540px;
  }

  h2 {
    text-align: center;
  }

  p {
    color: var(--color-foreground-level-5);
    margin: 16px 0 24px 0;
  }
</style>

<ModalLayout dataCy="page">
  <div class="wrapper">
    <div class="register-user" data-cy="register-user">
      {#if step === 1}
        <h2>Handle registration</h2>
        <p>
          Registering your handle makes it unique and allows others to easily
          find you.
        </p>
        <PickHandleStep bind:handle {identity} onNextStep={nextStep} />
      {/if}

      {#if step === 2}
        <SubmitRegistrationStep
          {identity}
          {transactionFee}
          onNextStep={registerUser}
          onPreviousStep={previousStep}
          {handle} />
      {/if}
    </div>
  </div>
</ModalLayout>
