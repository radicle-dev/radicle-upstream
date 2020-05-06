<script>
  import { getContext } from "svelte";
  import { pop } from "svelte-spa-router";

  import { Flex, Title } from "../DesignSystem/Primitive";
  import {
    NavigationButtons,
    ModalLayout,
    Remote,
    StepCounter,
    Transaction
  } from "../DesignSystem/Component";

  import RegistrationDetailsStep from "./ProjectRegistration/RegistrationDetailsStep.svelte";

  import { projects as projectStore } from "../src/project.ts";
  import { orgMocks } from "../lib/orgMocks.js";
  import * as transaction from "../src/transaction.ts";
  import * as project from "../src/project.ts";

  export let params = null;

  const session = getContext("session");

  let projectId = params.projectId || null;
  let registrarId = params.registrarId || null;

  let registrarHandle = null;
  let registrarAvatarFallback = null;
  let registrarImageUrl = null;
  let registrarVariant = null;

  let skipNamePreselection = false;

  let projectName = null;

  let showRegistrationDetails = true;

  // summary

  const onSubmitTransaction = () => {
    project.register(registrarHandle, projectName, projectId);

    pop();
  };

  const wallet = () => {
    return {
      name: registrarHandle,
      imageUrl: registrarImageUrl,
      avatarFallback: registrarAvatarFallback,
      variant: registrarVariant
    };
  };

  const subject = () => {
    return {
      name: `${registrarHandle} / ${projectName}`,
      imageUrl: registrarImageUrl,
      avatarFallback: registrarAvatarFallback,
      variant: registrarVariant
    };
  };

  const tx = {
    messages: [{ type: transaction.MessageType.ProjectRegistration }]
  };
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
  <ModalLayout>
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
            orgs={orgMocks.data.orgs}
            bind:projectId
            bind:registrarId
            bind:projectName
            on:next={event => {
              registrarHandle = event.detail.registrarHandle;
              registrarImageUrl = event.detail.registrarImageUrl;
              registrarAvatarFallback = event.detail.registrarAvatarFallback;
              registrarVariant = event.detail.registrarVariant;
              showRegistrationDetails = false;
            }} />
        {:else}
          <Transaction payer={wallet()} subject={subject()} transaction={tx} />

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
