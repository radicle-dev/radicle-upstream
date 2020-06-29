<script>
  import { pop, replace } from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import { State, store } from "../src/onboard.ts";
  import * as path from "../src/path.ts";
  import * as session from "../src/session.ts";

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

<Modal escapable={$store !== State.Welcome} {onClose}>
  {#if $store === State.Welcome}

  {:else if $store === State.Form}
    <Form
      on:cancel={returnToWelcomeStep}
      on:error={onError}
      on:success={() => store.set(State.SuccessView)} />
  {:else if $store === State.SuccessView}
    <Success on:close={onClose} on:register={onRegister} />
  {/if}
</Modal>
