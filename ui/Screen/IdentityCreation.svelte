<script>
  import { replace } from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import { State, store } from "../src/onboard.ts";
  import * as path from "../src/path.ts";
  import * as session from "../src/session.ts";

  import { ModalLayout } from "../DesignSystem/Component";

  import EnterName from "./IdentityCreation/EnterName.svelte";
  import Welcome from "./IdentityCreation/Welcome.svelte";
  import Success from "./IdentityCreation/Success.svelte";

  const returnToWelcomeStep = () => {
    store.set(State.Welcome);
  };

  const truncateUrn = message => {
    const urn = message.match(/(rad:git:\w{59})/)[1];

    if (urn) {
      return message.replace(/(rad:git:\w{59})/, urn.substr(-5));
    } else {
      return message;
    }
  };

  const onError = event => {
    notification.error(
      `Could not create identity: ${truncateUrn(event.detail.message)}`
    );
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
      case State.EnterName:
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

<ModalLayout escapable={false} {onClose}>
  {#if $store === State.Welcome}
    <Welcome on:next={() => store.set(State.EnterName)} />
  {:else if $store === State.EnterName}
    <EnterName
      on:cancel={returnToWelcomeStep}
      on:error={onError}
      on:success={() => store.set(State.SuccessView)} />
  {:else if $store === State.SuccessView}
    <Success on:close={onClose} on:register={onRegister} />
  {/if}
</ModalLayout>
