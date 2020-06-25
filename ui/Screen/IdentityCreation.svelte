<script>
  import { pop, replace } from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import { State, store } from "../src/onboard.ts";
  import * as path from "../src/path.ts";
  import * as session from "../src/session.ts";

  import Button from "../DesignSystem/Primitive/Button.svelte";
  import Text from "../DesignSystem/Primitive/Text.svelte";
  import RadicleLogo from "../DesignSystem/Component/RadicleLogo.svelte";

  import Modal from "../Layout/Modal.svelte";

  import Form from "./IdentityCreation/Form.svelte";
  import Success from "./IdentityCreation/Success.svelte";

  const returnToWelcomeStep = () => {
    store.set(State.Welcome);
  };

  const onError = event => {
    notification.error(`Could not create identity: ${event.detail}`);
    pop();
  };

  const complete = redirectPath => {
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

<Modal escapable={$store !== State.Welcome} {onClose}>
  {#if $store === State.Welcome}
    <div class="container">
      <RadicleLogo />
      <Text
        style="text-align: center; color: var(--color-foreground-level-5);
        margin: 2.5rem 0; max-width: 20rem;">
        A free and open-source way to host, share, and build software together.
      </Text>
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
</Modal>
