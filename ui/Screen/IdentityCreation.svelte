<script>
  import { pop, replace } from "svelte-spa-router";

  import * as path from "../lib/path.js";
  import { showNotification } from "../store/notification.js";
  import * as session from "../src/session.ts";
  import { CreationState, state } from "../src/identity.ts";

  import { ModalLayout, Placeholder } from "../DesignSystem/Component";
  import { Button } from "../DesignSystem/Primitive";

  import IdentityCreationForm from "./Identity/IdentityCreationForm.svelte";
  import IdentityCreationSuccess from "./Identity/IdentityCreationSuccess.svelte";

  const returnToWelcomeStep = () => {
    $state.set(CreationState.Welcome);
  };

  const onError = (error) => {
    pop();
    showNotification({
      text: `Could not create identity: ${error}`,
      level: "error",
    });
  };

  const onClose = () => {
    switch ($state) {
      case CreationState.Welcome:
        return;
      case CreationState.Form:
        returnToWelcomeStep();
        return;
      case CreationState.SuccessView:
        session.fetch();
        state.set(CreationState.Complete);
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

<ModalLayout hideCloseButton={$state === CreationState.Welcome} {onClose}>
  {#if $state === CreationState.Welcome}
    <div class="container">
      <Placeholder
        style="flex-shrink: 0; width: 800px; height: 400px; margin-bottom: 20px;" />
      <Button
        style="flex-shrink: 0; margin-bottom: 24px;"
        on:click={() => state.set(CreationState.Form)}
        dataCy="get-started-button">
        Get started
      </Button>
    </div>
  {:else if $state === CreationState.Form}
    <IdentityCreationForm
      onSuccess={() => state.set(CreationState.SuccessView)}
      {onError}
      onCancel={returnToWelcomeStep} />
  {:else if $state === CreationState.SuccessView}
    <IdentityCreationSuccess {onClose} />
  {/if}
</ModalLayout>
