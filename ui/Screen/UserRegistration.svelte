<script>
  import { gql } from "apollo-boost";
  import { getClient, mutate } from "svelte-apollo";
  import { pop } from "svelte-spa-router";

  import { identity } from "../src/identity.ts";
  import { showNotification } from "../store/notification.js";

  import { Text, Title } from "../DesignSystem/Primitive";
  import { ModalLayout, StepCounter } from "../DesignSystem/Component";

  import PickHandleStep from "./UserRegistration/PickHandleStep.svelte";
  import SubmitRegistrationStep from "./UserRegistration/SubmitRegistrationStep.svelte";

  let step = 1;

  let handle = $identity.handle;
  let avatarFallback = $identity.avatarFallback;
  const imageUrl = $identity.avatarUrl;
  const id = $identity.id;

  const nextStep = () => {
    step += 1;
  };

  const previousStep = () => {
    step -= 1;
  };

  const client = getClient();
  const REGISTER_USER = gql`
    mutation($handle: ID!, $id: ID!) {
      registerUser(handle: $handle, id: $id) {
        messages {
          ... on UserRegistrationMessage {
            handle
            id
          }
        }
      }
    }
  `;

  const registerUser = async () => {
    try {
      await mutate(client, {
        mutation: REGISTER_USER,
        variables: {
          handle: handle,
          id: id
        }
      });
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
        <PickHandleStep
          bind:avatarFallback
          {imageUrl}
          bind:handle
          onNextStep={nextStep} />
      {/if}

      {#if step === 2}
        <SubmitRegistrationStep
          onNextStep={registerUser}
          onPreviousStep={previousStep}
          {handle}
          {avatarFallback}
          {imageUrl} />
      {/if}
    </div>
  </div>
</ModalLayout>
