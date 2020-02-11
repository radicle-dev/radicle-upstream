<script>
  import { projectName } from "../stores.js";
  import { ModalLayout, StepCounter } from "../DesignSystem/Components";

  import PickNameStep from "./RegisterProject/PickNameStep.svelte";
  import PickWalletStep from "./RegisterProject/PickWalletStep.svelte";
  import ConfirmTransactionStep from "./RegisterProject/ConfirmTransactionStep.svelte";
  import TransactionSummaryStep from "./RegisterProject/TransactionSummaryStep.svelte";

  import { Text } from "../DesignSystem/Primitives";
  import { gql } from "apollo-boost";
  import { getClient, mutate } from "svelte-apollo";

  const stepTitle = {
    1: "Register your project",
    2: "Pick a wallet",
    3: "Confirm transaction",
    4: "Transaction submitted"
  };

  let step = 1;
  let name = $projectName;

  const nextStep = () => {
    step += 1;
  };

  const previousStep = () => {
    step -= 1;
  };

  const client = getClient();

  const REGISTER_PROJECT = gql`
    mutation($domain: String!, $name: String!) {
      registerProject(domain: $domain, name: $name) {
        id
        messages {
          ... on ProjectRegistration {
            domain
            name
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

  let response;
  let errorMessage;
  const registerProject = async () => {
    try {
      response = await mutate(client, {
        mutation: REGISTER_PROJECT,
        variables: {
          name: name,
          domain: "rad"
        }
      });
    } catch (error) {
      errorMessage = error;
    } finally {
      nextStep();
    }
  };

  const formatDate = date => {
    let options = {
      hour: "numeric",
      minute: "numeric",
      day: "numeric",
      month: "long",
      year: "numeric"
    };
    return new Intl.DateTimeFormat("en-US", options).format(date);
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
        <StepCounter {step} of={4} style="margin-bottom: 16px" />
      </div>

      <Text variant="bigTitle" style="margin-bottom: 24px; text-align: center">
        {stepTitle[step]}
      </Text>

      {#if step === 1}
        <PickNameStep bind:name onNextStep={nextStep} />
      {/if}

      {#if step === 2}
        <PickWalletStep onNextStep={nextStep} onPreviousStep={previousStep} />
      {/if}

      {#if step === 3}
        <ConfirmTransactionStep
          onNextStep={registerProject}
          onPreviousStep={previousStep}
          {name} />
      {/if}

      {#if step === 4}
        {#if response}
          <TransactionSummaryStep
            name={response.data.registerProject.messages[0].name}
            timestamp={formatDate(response.data.registerProject.timestamp * 1000)} />
        {:else}
          <TransactionSummaryStep {name} {errorMessage} />
        {/if}
      {/if}
    </div>
  </div>
</ModalLayout>
