<script>
  import { getContext } from "svelte";
  import { pop } from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import { register, projects as projectStore } from "../src/project.ts";
  import { fetch as fetchSession } from "../src/session.ts";
  import * as transaction from "../src/transaction.ts";

  import { Flex, Title } from "../DesignSystem/Primitive";
  import {
    NavigationButtons,
    Remote,
    StepCounter,
    Transaction,
  } from "../DesignSystem/Component";

  import Modal from "../Layout/Modal.svelte";

  import RegistrationDetailsStep from "./ProjectRegistration/RegistrationDetailsStep.svelte";

  export let params = null;

  const session = getContext("session");

  let projectId = params.projectId || null;
  let projectName = null;

  let domainId = params.domainId || null;
  let domainType = null;
  let domainAvatar = null;

  let skipNamePreselection = false;
  let showRegistrationDetails = true;

  const transactionFee = session.minimumTransactionFee;

  const registerProject = async () => {
    try {
      await register(
        domainType,
        domainId,
        projectName,
        transactionFee,
        projectId
      );
      await fetchSession();
    } catch (error) {
      notification.error(`Could not register project: ${error.message}`);
    } finally {
      pop();
    }
  };

  const wallet = () => transaction.formatPayer(session.identity);

  // TODO(sos): coordinate message format for project registration with proxy
  // See https://github.com/radicle-dev/radicle-upstream/issues/441
  const tx = () => ({
    fee: transactionFee,
    messages: [
      {
        type: transaction.MessageType.ProjectRegistration,
        domainType: domainType,
        domainId: domainId,
        projectName: projectName,
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
  <Modal dataCy="project-registration-screen">
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
            identity={session.identity}
            {projects}
            {skipNamePreselection}
            orgs={session.orgs}
            bind:projectId
            bind:domainId
            bind:projectName
            on:next={event => {
              domainId = event.detail.domainId;
              domainType = event.detail.domainType;
              showRegistrationDetails = false;
              domainAvatar = event.detail.domainAvatar;
            }} />
        {:else}
          <Transaction
            transaction={tx()}
            payer={wallet()}
            transactionDeposits={session.transactionDeposits} />

          <NavigationButtons
            style={'margin-top: 32px;'}
            cancelLabel="Back"
            submitLabel="Submit transaction"
            on:cancel={() => {
              showRegistrationDetails = true;
              skipNamePreselection = true;
            }}
            on:submit={registerProject} />
        {/if}
      </div>
    </div>
  </Modal>
</Remote>
