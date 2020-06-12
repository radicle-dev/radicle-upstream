<script>
  import { getContext } from "svelte";
  import { pop } from "svelte-spa-router";

  import { Flex, Title } from "../DesignSystem/Primitive";
  import {
    NavigationButtons,
    ModalLayout,
    Remote,
    StepCounter,
    Transaction,
  } from "../DesignSystem/Component";

  import Form from "./ProjectRegistration/Form.svelte";

  import {
    formatRegistrantOptions,
    formatTransaction,
    register,
    projects as projectStore,
    RegistrationState,
  } from "../src/project.ts";
  import * as transaction from "../src/transaction.ts";

  export let params = null;

  const session = getContext("session");

  let projectId = params.projectId || null;
  let projectName = null;

  let registrantId = params.registrantId || null;

  const skipNamePreselection = false;
  let state = RegistrationState.Preparation,
    valid = false;

  const wallet = () => transaction.formatPayer(session.identity);

  const registrantOptions = formatRegistrantOptions(
    session.identity,
    session.orgs
  );

  const selectedRegistrant = () =>
    registrantOptions.find((option) => option.value === registrantId);

  const next = () => {
    switch (state) {
      case RegistrationState.Preparation:
        if (valid) state = RegistrationState.Confirmation;
        break;
      case RegistrationState.Confirmation:
        register(registrantId, projectName, projectId);
        pop();
    }
  };

  const cancel = () => {
    switch (state) {
      case RegistrationState.Preparation:
        pop();
        break;
      case RegistrationState.Confirmation:
        state = RegistrationState.Preparation;
    }
  };

  $: submitLabel =
    state === RegistrationState.Confirmation ? "Submit transaction" : "Next";
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    margin: 92px 0 72px 0;
  }

  .project-registration {
    width: 540px;
  }
</style>

<Remote store={projectStore} let:data={projects}>
  <ModalLayout dataCy="project-registration-screen">
    <div class="wrapper">
      <div class="project-registration">
        <Flex align="center" style="margin-bottom: 40px;">
          <StepCounter
            selectedStep={state + 1}
            steps={['Prepare', 'Submit']}
            style="margin-bottom: 48px" />

          <Title variant="big">Register project</Title>
        </Flex>

        {#if state === RegistrationState.Preparation}
          <Form
            {registrantOptions}
            {projects}
            {skipNamePreselection}
            bind:projectId
            bind:projectName
            bind:registrantId
            bind:valid />
        {:else if state === RegistrationState.Confirmation}
          <Transaction
            transaction={formatTransaction(projectName, selectedRegistrant())}
            payer={wallet()} />
        {/if}
        <NavigationButtons
          style={'margin-top: 32px;'}
          cancelLabel={state === RegistrationState.Preparation ? 'Cancel' : 'Back'}
          submitLabel={state === RegistrationState.Preparation ? 'Next' : 'Submit transaction'}
          on:cancel={cancel}
          on:submit={next}
          disableSubmit={!valid} />
      </div>
    </div>
  </ModalLayout>
</Remote>
