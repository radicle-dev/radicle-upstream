<script>
  import { pop, replace } from "svelte-spa-router";

  import { showNotification } from "../store/notification.js";
  import { ModalLayout } from "../DesignSystem/Component";
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

  const onError = error => {
    pop();
    showNotification({
      text: `Could not create identity: ${error}`,
      level: "error"
    });
  };
</script>

<style>
  .container {
    display: flex;
    justify-content: center;
    margin-top: 633px;
  }
</style>

<ModalLayout>
  {#if currentStep === steps.WELCOME}
    <div class="container">
      <Button on:click={nextStep}>Get started</Button>
    </div>
  {:else if currentStep === steps.FORM}
    <IdentityCreationForm onSuccess={nextStep} {onError} />
  {:else if currentStep === steps.SUCCESS}
    <IdentityCreationSuccess onClose={() => replace('/projects')} />
  {/if}
</ModalLayout>
