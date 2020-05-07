<script>
  import { pop, replace } from "svelte-spa-router";

  import * as path from "../lib/path.js";
  import { showNotification } from "../store/notification.js";
  import * as session from "../src/session.ts";
  import { LaunchFlowState, launchFlowStore } from "../src/identity.ts";

  import { ModalLayout, Placeholder } from "../DesignSystem/Component";
  import { Button } from "../DesignSystem/Primitive";

  import IdentityCreationForm from "./Identity/IdentityCreationForm.svelte";
  import IdentityCreationSuccess from "./Identity/IdentityCreationSuccess.svelte";

  const returnToWelcomeStep = () => {
    $launchFlowStore.set(LaunchFlowState.Welcome);
  };

  const onError = (error) => {
    pop();
    showNotification({
      text: `Could not create identity: ${error}`,
      level: "error",
    });
  };

  const onClose = () => {
    switch ($launchFlowStore) {
      case LaunchFlowState.Welcome:
        return;
      case LaunchFlowState.Form:
        returnToWelcomeStep();
        return;
      case LaunchFlowState.SuccessView:
        session.fetch();
        launchFlowStore.set(LaunchFlowState.Complete);
        replace(path.profileProjects());
        return;
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

<ModalLayout
  hideCloseButton={$launchFlowStore === LaunchFlowState.Welcome}
  {onClose}>
  {#if $launchFlowStore === LaunchFlowState.Welcome}
    <div class="container">
      <Placeholder
        style="flex-shrink: 0; width: 800px; height: 400px; margin-bottom: 20px;" />
      <Button
        style="flex-shrink: 0; margin-bottom: 24px;"
        on:click={() => launchFlowStore.set(LaunchFlowState.Form)}
        dataCy="get-started-button">
        Get started
      </Button>
    </div>
  {:else if $launchFlowStore === LaunchFlowState.Form}
    <IdentityCreationForm
      onSuccess={() => launchFlowStore.set(LaunchFlowState.SuccessView)}
      {onError}
      onCancel={returnToWelcomeStep} />
  {:else if $launchFlowStore === LaunchFlowState.SuccessView}
    <IdentityCreationSuccess {onClose} />
  {/if}
</ModalLayout>
