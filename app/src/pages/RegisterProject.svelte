<script>
  import { projectName } from "../stores.js";
  import ModalLayout from "../layouts/ModalLayout.svelte";
  import StepCounter from "../components/ProjectRegistration/StepCounter.svelte";

  import StepOne from "../components/ProjectRegistration/StepOne.svelte";
  import StepTwo from "../components/ProjectRegistration/StepTwo.svelte";
  import StepThree from "../components/ProjectRegistration/StepThree.svelte";
  import StepFour from "../components/ProjectRegistration/StepFour.svelte";

  import { Title } from "../DesignSystem";

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

      <Title.Big style="margin-bottom: 24px; text-align: center">
        {stepTitle[step]}
      </Title.Big>

      {#if step === 1}
        <StepOne bind:name onNextStep={nextStep} />
      {/if}

      {#if step === 2}
        <StepTwo onNextStep={nextStep} onPreviousStep={previousStep} />
      {/if}

      {#if step === 3}
        <StepThree onNextStep={nextStep} onPreviousStep={previousStep} {name} />
      {/if}

      {#if step === 4}
        <StepFour {name} />
      {/if}
    </div>
  </div>
</ModalLayout>
