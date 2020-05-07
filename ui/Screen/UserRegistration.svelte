<script>
  import { getContext } from "svelte";
  import { pop } from "svelte-spa-router";

  import { fallback } from "../src/identity.ts";
  import * as notification from "../src/notification.ts";
  import * as user from "../src/user.ts";

  import { ModalLayout, StepCounter } from "../DesignSystem/Component";
  import { Text, Title } from "../DesignSystem/Primitive";

  import PickHandleStep from "./UserRegistration/PickHandleStep.svelte";
  import SubmitRegistrationStep from "./UserRegistration/SubmitRegistrationStep.svelte";

  const session = getContext("session");

  let step = 1;

  let identity = fallback;
  let handle = null;
  let id = null;

  if (session.identity !== null) {
    identity = session.identity;
    handle = session.identity.metadata.handle;
    id = session.identity.id;
  }

  const nextStep = () => {
    step += 1;
  };

  const previousStep = () => {
    step -= 1;
  };

  const registerUser = async () => {
    try {
      await user.register(handle, id);
    } catch (error) {
      notification.error({ message: `Could not register user: ${error}` });
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
</style>

<ModalLayout dataCy="page">
  <div class="wrapper">
    <div class="register-user" data-cy="register-user">
      <div style="display: flex; justify-content: center">
        <StepCounter
          selectedStep={step}
          steps={['Prepare', 'Submit']}
          style="margin-bottom: 16px" />
      </div>

      <Title variant="big" style="margin: 48px 0 24px 0; text-align: center">
        Register your handle
      </Title>

      {#if step === 1}
        <Text
          style="color: var(--color-foreground-level-5); margin: 16px 0 24px 0;">
          Registering your handle makes it unique and allows others to easily
          find you.
        </Text>
        <PickHandleStep bind:handle {identity} onNextStep={nextStep} />
      {/if}

      {#if step === 2}
        <SubmitRegistrationStep
          {identity}
          onNextStep={registerUser}
          onPreviousStep={previousStep}
          {handle} />
      {/if}
    </div>
  </div>
</ModalLayout>
