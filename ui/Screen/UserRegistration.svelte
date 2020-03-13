<script>
  import { gql } from "apollo-boost";
  import { getClient, mutate } from "svelte-apollo";
  import { pop } from "svelte-spa-router";

  import { Text, Title } from "../DesignSystem/Primitive";
  import { ModalLayout, StepCounter } from "../DesignSystem/Component";

  import PickHandleStep from "./UserRegistration/PickHandleStep.svelte";
  import SubmitRegistrationStep from "./UserRegistration/SubmitRegistrationStep.svelte";

  let step = 1;
  // TODO(merle): Get actual user profile (id, name, imageUrl, avatarFallback)
  let handle = "cloudhead";
  const avatarFallback = {
    emoji: "📐",
    background: {
      r: 24,
      g: 105,
      b: 216
    }
  };
  const imageUrl = null;

  const id = "1234";

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
          ... on UserRegistration {
            handle
            id
          }
        }
      }
    }
  `;

  let response;
  let errorMessage;
  const registerUser = async () => {
    // TODO(merle): Remove log statements and add error and success handling
    // once tx handling is in place
    console.log("Register handle, id ", handle, id);
    try {
      response = await mutate(client, {
        mutation: REGISTER_USER,
        variables: {
          handle: handle,
          id: id
        }
      });
    } catch (error) {
      errorMessage = error;
      console.log("Register user error: ", errorMessage);
    } finally {
      console.log("Register user respone", response);
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

<ModalLayout>
  <div class="wrapper">
    <div class="register-user">
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
        <Text style="color: var(--color-gray); margin: 16px 0 24px 0;">
          Registering your handle makes it unique and allows others to easily
          find you.
        </Text>
        <PickHandleStep
          {avatarFallback}
          {imageUrl}
          bind:handle
          onNextStep={nextStep} />
      {/if}

      {#if step === 2}
        <SubmitRegistrationStep
          onNextStep={registerUser}
          onPreviousStep={previousStep}
          {handle} />
      {/if}
    </div>
  </div>
</ModalLayout>
