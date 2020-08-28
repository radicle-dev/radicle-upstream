<script>
  import { replace } from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import { State, store } from "../src/onboard.ts";
  import * as path from "../src/path.ts";
  import * as session from "../src/session.ts";

  import { ModalLayout } from "../DesignSystem/Component";

  import Welcome from "./Onboarding/Welcome.svelte";
  import EnterName from "./Onboarding/EnterName.svelte";
  import EnterPassphrase from "./Onboarding/EnterPassphrase.svelte";
  import Success from "./Onboarding/Success.svelte";

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
    store.set(State.Welcome);
    replace(redirectPath);
  };
</script>

<ModalLayout escapable={false}>
  {#if $store === State.Welcome}
    <Welcome on:next={() => store.set(State.EnterName)} />
  {:else if $store === State.EnterName}
    <EnterName
      on:cancel={returnToWelcomeStep}
      on:error={onError}
      on:next={() => store.set(State.EnterPassphrase)} />
  {:else if $store === State.EnterPassphrase}
    <EnterPassphrase
      on:cancel={returnToWelcomeStep}
      on:next={() => store.set(State.SuccessView)} />
  {:else if $store === State.SuccessView}
    <Success
      on:close={() => {
        complete(path.profileProjects());
      }} />
  {/if}
</ModalLayout>
