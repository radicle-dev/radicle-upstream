<script>
  import {
    IdentifierSelectionStep,
    ModalLayout,
    StepCounter,
    Transaction
  } from "../DesignSystem/Component";
  import { Title } from "../DesignSystem/Primitive";

  const steps = {
    PREPARE: 1,
    SUBMIT: 2
  };

  let currentStep = steps.PREPARE;

  const nextStep = () => currentStep++;

  const imageUrl =
    "https://pbs.twimg.com/profile_images/378800000411356732/e8b1b7f0bd07d4d948cb2da25e221673_400x400.jpeg";

  const transaction = {
    message: "Org registration",
    stake: "Org registration deposit",
    subject: {
      name: "my org",
      kind: "org",
      avatarFallback: null,
      imageUrl: imageUrl
    },
    payer: {
      name: "someone",
      kind: "org",
      avatarFallback: null,
      imageUrl: imageUrl
    }
  };
</script>

<style>
  .container {
    margin: 86px 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
</style>

<ModalLayout>
  <div class="container">
    <StepCounter
      selectedStep={currentStep}
      steps={['Prepare', 'Submit']}
      style="margin-bottom: 50px;" />
    <Title variant="big" style="margin-bottom: 16px;">Register an org</Title>
    {#if currentStep === steps.PREPARE}
      <IdentifierSelectionStep
        explanatoryText="Registering an org allows you to give others in your
        org the right to sign transactions, like adding other members or adding
        projects."
        inputPlaceholder="Org name (e.g. Flowerpot)"
        entity="Org name"
        onNextStep={nextStep} />
    {:else if currentStep === steps.SUBMIT}
      <div>
        <Transaction {transaction} />
      </div>
    {/if}
  </div>
</ModalLayout>
