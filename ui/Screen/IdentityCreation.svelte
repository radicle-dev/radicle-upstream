<script>
  import { pop, replace } from "svelte-spa-router";

  import * as path from "../lib/path.js";
  import { showNotification } from "../store/notification.js";
  import { ModalLayout, Placeholder } from "../DesignSystem/Component";
  import { Button } from "../DesignSystem/Primitive";

  import IdentityCreationForm from "./Identity/IdentityCreationForm.svelte";
  import IdentityCreationSuccess from "./Identity/IdentityCreationSuccess.svelte";

  const steps = {
    WELCOME: 1,
    FORM: 2,
    SUCCESS: 3
  };

  let currentStep = steps.WELCOME;

  const nextStep = () => {
    currentStep += 1;
  };

  const returnToWelcomeStep = () => {
    currentStep = steps.WELCOME;
  };

  const onError = error => {
    pop();
    showNotification({
      text: `Could not create identity: ${error}`,
      level: "error"
    });
  };

  const onClose = () => {
    switch (currentStep) {
      case steps.WELCOME:
        return;
      case steps.FORM:
        returnToWelcomeStep();
        return;
      case steps.SUCCESS:
        pop();
    }
  };
</script>

<style>
  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    height: 100%;
  }
</style>

<ModalLayout hideCloseButton={currentStep === steps.WELCOME} {onClose}>
  {#if currentStep === steps.WELCOME}
    <div class="container">
      <Placeholder
        style="flex-shrink: 0; width: 800px; height: 400px; margin-bottom: 20px;" />
      <Button
        style="flex-shrink: 0; margin-bottom: 24px;"
        on:click={nextStep}
        dataCy="get-started-button">
        Get started
      </Button>
    </div>
  {:else if currentStep === steps.FORM}
    <IdentityCreationForm
      onSuccess={nextStep}
      {onError}
      onCancel={returnToWelcomeStep} />
  {:else if currentStep === steps.SUCCESS}
    <IdentityCreationSuccess onClose={() => replace(path.profileProjects())} />
  {/if}
</ModalLayout>
