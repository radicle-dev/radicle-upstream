<script>
  import { Title } from "../DesignSystem/Primitive";
  import { ModalLayout, StepCounter } from "../DesignSystem/Component";

  import PickNewOrExistingProjectStep from "./ProjectRegistration/PickNewOrExistingProjectStep.svelte";
  import RegistrationDetailsStep from "./ProjectRegistration/RegistrationDetailsStep.svelte";
  import TransactionSummaryStep from "./ProjectRegistration/TransactionSummaryStep.svelte";

  export let params = null;

  let createNewProject = null;
  let projectId = params.projectId || null;
  let registrarId = params.registrarId || null;

  const steps = {
    NEW_OR_EXISTING: 1,
    DETAILS: 2,
    SUMMARY: 3
  };

  let currentStep = projectId === null ? steps.NEW_OR_EXISTING : steps.DETAILS;

  const nextStep = () => {
    currentStep += 1;
  };
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    margin: 92px 0 72px 0;
  }

  .project-registration {
    text-align: center;
    flex: 1;
  }
</style>

<ModalLayout>
  <div class="wrapper">
    <div class="project-registration">
      <StepCounter
        selectedStep={currentStep === steps.SUMMARY ? 2 : 1}
        steps={['Prepare', 'Submit']}
        style="margin-bottom: 48px" />

      <Title variant="big" style="margin-bottom: 24px;">Register project</Title>

      {#if currentStep === steps.NEW_OR_EXISTING}
        <PickNewOrExistingProjectStep
          bind:createNewProject
          on:next={() => {
            nextStep();
          }} />
      {:else if currentStep === steps.DETAILS}
        <RegistrationDetailsStep
          bind:projectId
          bind:registrarId
          {createNewProject}
          on:next={() => {
            nextStep();
          }} />
      {:else if currentStep === steps.SUMMARY}
        <TransactionSummaryStep {projectId} {registrarId} />
      {/if}

    </div>
  </div>
</ModalLayout>
