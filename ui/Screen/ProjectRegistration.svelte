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

  import RegistrationDetailsStep from "./ProjectRegistration/RegistrationDetailsStep.svelte";

  import { projects as projectStore } from "../src/project.ts";
  import * as transaction from "../src/transaction.ts";
  import * as project from "../src/project.ts";

  export let params = null;

  const session = getContext("session");

  let projectId = params.projectId || null;
  let projectName = null;

  let domainId = params.domainId || null;
  let domainType = null;
  let domainAvatar = null;

  let skipNamePreselection = false;
  let showRegistrationDetails = true;

  // summary

  const onSubmitTransaction = () => {
    project.register(domainId, projectName, projectId);

    pop();
  };

  const wallet = () => transaction.formatPayer($session.data.identity);

  // TODO(sos): coordinate message format for project registration with proxy
  const tx = () => ({
    messages: [
      {
        type: transaction.MessageType.ProjectRegistration,
        // domain: domainType,
        org_id: domainId,
        project_name: projectName,
      },
    ],
  });
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
            selectedStep={showRegistrationDetails ? 1 : 2}
            steps={['Prepare', 'Submit']}
            style="margin-bottom: 48px" />

          <Title variant="big">Register project</Title>
        </Flex>

        {#if showRegistrationDetails === true}
          <RegistrationDetailsStep
            {session}
            {projects}
            {skipNamePreselection}
            orgs={session.orgs}
            bind:projectId
            bind:domainId
            bind:projectName
            on:next={(event) => {
              domainId = event.detail.domainId;
              domainType = event.detail.domainType;
              showRegistrationDetails = false;
              domainAvatar = event.detail.domainAvatar;
            }} />
        {:else}
          <Transaction transaction={tx()} payer={wallet()} />

          <NavigationButtons
            style={'margin-top: 32px;'}
            cancelLabel="Back"
            submitLabel="Submit transaction"
            on:cancel={() => {
              showRegistrationDetails = true;
              skipNamePreselection = true;
            }}
            on:submit={onSubmitTransaction} />
        {/if}
      </div>
    </div>
  </ModalLayout>
</Remote>
