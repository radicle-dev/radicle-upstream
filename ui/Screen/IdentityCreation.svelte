<script>
  import { pop, replace } from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import { State, store } from "../src/onboard.ts";
  import * as path from "../src/path.ts";
  import * as session from "../src/session.ts";

  import { ModalLayout, Placeholder } from "../DesignSystem/Component";
  import { Button } from "../DesignSystem/Primitive";

  import Form from "./IdentityCreation/Form.svelte";
  import Success from "./IdentityCreation/Success.svelte";

  const returnToWelcomeStep = () => {
    store.set(State.Welcome);
  };

  const onError = (error) => {
    pop();
    notification.error(`Could not create identity: ${error}`);
  };

  const complete = (redirectPath) => {
    session.fetch();
    store.set(State.Complete);
    replace(redirectPath);
  };

  const onClose = () => {
    switch ($store) {
      case State.Welcome:
        return;
      case State.Form:
        returnToWelcomeStep();
        return;
      case State.SuccessView:
        complete(path.profileProjects());
        return;
    }
  };

  const onRegister = () => {
    replace(path.profileProjects());
    complete(path.registerUser());
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

<ModalLayout escapable={$store !== State.Welcome} {onClose}>
  {#if $store === State.Welcome}
    <div class="container">
      <Placeholder
        style="flex-shrink: 0; width: 800px; height: 400px; margin-bottom: 20px;" />
      <Button
        style="flex-shrink: 0; margin-bottom: 24px;"
        on:click={() => store.set(State.Form)}
        dataCy="get-started-button">
        Get started
      </Button>
    </div>
  {:else if $store === State.Form}
    <Form
      on:cancel={returnToWelcomeStep}
      on:error={onError}
      on:success={() => store.set(State.SuccessView)} />
  {:else if $store === State.SuccessView}
    <Success on:close={onClose} on:register={onRegister} />
  {/if}
</ModalLayout>
