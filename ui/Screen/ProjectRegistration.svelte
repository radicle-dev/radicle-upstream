<script>
  import { Flex, Title } from "../DesignSystem/Primitive";
  import { ModalLayout, Remote, StepCounter } from "../DesignSystem/Component";

  import RegistrationDetailsStep from "./ProjectRegistration/RegistrationDetailsStep.svelte";
  import TransactionSummaryStep from "./ProjectRegistration/TransactionSummaryStep.svelte";

  import { projects as projectStore } from "../src/project.ts";
  import { session as sessionStore } from "../src/session.ts";
  import { orgMocks } from "../lib/orgMocks.js";

  export let params = null;

  let projectId = params.projectId || null;
  let registrarId = params.registrarId || null;

  let registrarHandle = null;
  let registrarAvatarFallback = null;
  let registrarImageUrl = null;
  let registrarVariant = null;

  let skipNamePreselection = false;

  let projectName = null;

  let showRegistrationDetails = true;
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
  <Remote store={sessionStore} let:data={session}>
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
            <TransactionSummaryStep
              on:previous={() => {
                showRegistrationDetails = true;
                skipNamePreselection = true;
              }}
              {projectId}
              {registrarHandle}
              {registrarImageUrl}
              {registrarAvatarFallback}
              {registrarVariant}
              {projectName} />
          {/if}
        </div>
      </div>
    </ModalLayout>
  </Remote>
</Remote>
