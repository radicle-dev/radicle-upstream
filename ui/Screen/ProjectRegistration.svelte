<script>
  import { Flex, Title } from "../DesignSystem/Primitive";
  import { ModalLayout, StepCounter } from "../DesignSystem/Component";

  import RegistrationDetailsStep from "./ProjectRegistration/RegistrationDetailsStep.svelte";
  import TransactionSummaryStep from "./ProjectRegistration/TransactionSummaryStep.svelte";

  export let params = null;

  let projectId = params.projectId || null;
  let registrarId = params.registrarId || null;

  const steps = {
    DETAILS: 1,
    SUMMARY: 2
  };

  let currentStep = steps.DETAILS;

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
    width: 540px;
  }
</style>

<ModalLayout>
  <div class="wrapper">
    <div class="project-registration">
      <Flex align="center" style="margin-bottom: 40px;">
        <StepCounter
          selectedStep={currentStep}
          steps={['Prepare', 'Submit']}
          style="margin-bottom: 48px" />

        <Title variant="big">Register project</Title>
      </Flex>

      {#if currentStep === steps.DETAILS}
        <RegistrationDetailsStep
          bind:projectId
          bind:registrarId
          on:next={() => {
            nextStep();
          }} />
      {:else}
        <TransactionSummaryStep {projectId} {registrarId} />
      {/if}
    </div>
  </div>
</ModalLayout>
