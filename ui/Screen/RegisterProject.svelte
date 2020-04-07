<script>
  import { gql } from "apollo-boost";
  import { getClient, mutate } from "svelte-apollo";
  import { pop } from "svelte-spa-router";

  import { showNotification } from "../store/notification.js";
  import { projectNameStore } from "../store/project.ts";

  import { Title } from "../DesignSystem/Primitive";
  import { ModalLayout, StepCounter } from "../DesignSystem/Component";

  import PickNameStep from "./RegisterProject/PickNameStep.svelte";
  import PickWalletStep from "./RegisterProject/PickWalletStep.svelte";
  import ConfirmTransactionStep from "./RegisterProject/ConfirmTransactionStep.svelte";

  const stepTitle = {
    1: "Register your project",
    2: "Pick a wallet",
    3: "Confirm transaction"
  };

  let step = 1;
  let projectName = $projectNameStore;
  const orgId = "monadic"; // TODO(rudolfs): get the proper org id!

  const nextStep = () => {
    step += 1;
  };

  const previousStep = () => {
    step -= 1;
  };

  const client = getClient();

  const REGISTER_PROJECT = gql`
    mutation($orgId: String!, $projectName: String!) {
      registerProject(orgId: $orgId, projectName: $projectName) {
        id
        messages {
          ... on ProjectRegistrationMessage {
            projectName
            orgId
          }
        }
        state {
          ... on Applied {
            block
          }
        }
        timestamp
      }
    }
  `;

  const registerProject = async () => {
    try {
      await mutate(client, {
        mutation: REGISTER_PROJECT,
        variables: {
          projectName: projectName,
          orgId: orgId
        }
      });
    } catch (error) {
      showNotification({
        text: `Could not register project: ${error}`,
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

  .register-project {
    text-align: left;
    width: 540px;
  }
</style>

<ModalLayout>
  <div class="wrapper">
    <div class="register-project">
      <div style="display: flex; justify-content: center">
        <StepCounter
          selectedStep={step}
          steps={[1, 2, 3]}
          style="margin-bottom: 16px" />
      </div>

      <Title variant="big" style="margin-bottom: 24px; text-align: center">
        {stepTitle[step]}
      </Title>

      {#if step === 1}
        <PickNameStep bind:projectName onNextStep={nextStep} />
      {/if}

      {#if step === 2}
        <PickWalletStep onNextStep={nextStep} onPreviousStep={previousStep} />
      {/if}

      {#if step === 3}
        <ConfirmTransactionStep
          onNextStep={registerProject}
          onPreviousStep={previousStep}
          {projectName} />
      {/if}
    </div>
  </div>
</ModalLayout>
