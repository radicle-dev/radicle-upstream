<script>
  import { Title } from "../DesignSystem/Primitive";
  import { ModalLayout, StepCounter } from "../DesignSystem/Component";

  import PickNewOrExistingProjectStep from "./ProjectRegistration/PickNewOrExistingProjectStep.svelte";
  import RegistrationDetailsStep from "./ProjectRegistration/RegistrationDetailsStep.svelte";
  import TransactionSummaryStep from "./ProjectRegistration/TransactionSummaryStep.svelte";

  export let params = null;

  const projectId = params.projectId || null;
  const registrarId = params.registrarId || null;

  const steps = {
    NEW_OR_EXISTING: 1,
    DETAILS: 2,
    SUMMARY: 3
  };

  let currentStep = projectId === null ? steps.NEW_OR_EXISTING : steps.DETAILS;

  const nextStep = () => {
    currentStep += 1;
  };

  let createNewProject = null;

  console.log(projectId);
  console.log(registrarId);
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
          onNextStep={nextStep} />
      {:else if currentStep === steps.DETAILS}
        <RegistrationDetailsStep
          {projectId}
          {createNewProject}
          onNextStep={nextStep} />
      {:else if currentStep === steps.SUMMARY}
        <TransactionSummaryStep />
      {/if}

    </div>
  </div>
</ModalLayout>
