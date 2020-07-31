<script>
  import { getContext } from "svelte";
  import { pop } from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import { register, projects as projectStore } from "../src/project.ts";
  import { fetch as fetchSession } from "../src/session.ts";
  import * as transaction from "../src/transaction.ts";

  import { Title } from "../DesignSystem/Primitive";
  import {
    NavigationButtons,
    ModalLayout,
    Remote,
    Transaction,
  } from "../DesignSystem/Component";

  import RegistrationDetailsStep from "./ProjectRegistration/RegistrationDetailsStep.svelte";

  export let params = null;

  const session = getContext("session");

  let projectId = params.projectId || null;
  let projectName = null;

  let domainId = params.domainId || null;
  let domainType = null;

  let skipNamePreselection = false;
  let showRegistrationDetails = true;

  // The `transactionFee` will be user-customizable in the future.
  const transactionFee = session.minimumTransactionFee;
  const registrationFee = session.registrationFee.project;

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

  // TODO(nuno): payer should be built from domainId
  const payer = () => transaction.formatPayer(session.identity);

  // TODO(sos): coordinate message format for project registration with proxy
  // See https://github.com/radicle-dev/radicle-upstream/issues/441
  const tx = () => ({
    fee: transactionFee,
    registrationFee: registrationFee,
    messages: [
      {
        type: transaction.MessageType.ProjectRegistration,
        domainType: domainType,
        domainId: domainId,
        projectName: projectName,
      },
    ],
  });

  const handleDetailsNextClick = event => {
    domainId = event.detail.domainId;
    domainType = event.detail.domainType;
    showRegistrationDetails = false;
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
  <ModalLayout dataCy="project-registration-screen">
    <div class="wrapper">
      <div class="project-registration">

        {#if showRegistrationDetails === true}
          <Title variant="big" style="text-align: center; margin-bottom: 24px;">
            Project registration
          </Title>
          <RegistrationDetailsStep
            identity={session.identity}
            {projects}
            {skipNamePreselection}
            orgs={session.orgs}
            bind:projectId
            bind:domainId
            bind:projectName
            on:next={handleDetailsNextClick} />
        {:else}
          <Transaction transaction={tx()} payer={payer()} />

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
  </ModalLayout>
</Remote>
