<script>
  import { pop } from "svelte-spa-router";

  import { fallback } from "../src/identity.ts";
  import { showNotification } from "../store/notification.js";
  import * as remote from "../src/remote.ts";
  import { session } from "../src/session.ts";
  import * as user from "../src/user.ts";

  import { ModalLayout, StepCounter } from "../DesignSystem/Component";
  import { Text, Title } from "../DesignSystem/Primitive";

  import PickHandleStep from "./UserRegistration/PickHandleStep.svelte";
  import SubmitRegistrationStep from "./UserRegistration/SubmitRegistrationStep.svelte";

  let step = 1;

  let identity = fallback;
  let handle = null;
  let id = null;

  if (
    $session.status === remote.Status.Success &&
    $session.data.identity !== null
  ) {
    identity = $session.data.identity;
    handle = $session.data.identity.metadata.handle;
    id = $session.data.identity.id;
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
      showNotification({
        text: `Could not register user: ${error}`,
        level: "error"
      });
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
