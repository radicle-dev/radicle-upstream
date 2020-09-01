<script>
  import { replace } from "svelte-spa-router";

  import * as notification from "../src/notification.ts";
  import { State } from "../src/onboarding.ts";
  import { createIdentity } from "../src/identity.ts";
  import * as path from "../src/path.ts";
  import * as session from "../src/session.ts";
  import * as urn from "../src/urn.ts";

  import { ModalLayout } from "../DesignSystem/Component";

  import Welcome from "./Onboarding/Welcome.svelte";
  import EnterName from "./Onboarding/EnterName.svelte";
  import EnterPassphrase from "./Onboarding/EnterPassphrase.svelte";
  import Success from "./Onboarding/Success.svelte";

  let identity;
  let handle;
  let state = State.Welcome;

  const complete = () => {
    session.fetch();
    state = State.Welcome;
    replace(path.profileProjects());
  };

  const onCreateIdentity = async (handle, passphrase) => {
    try {
      identity = await createIdentity({
        handle: handle,
        passphrase: passphrase,
      });
      state = State.SuccessView;
    } catch (error) {
      state = State.EnterName;
      notification.error(
        `Could not create identity: ${urn.shorten(error.message)}`
      );
    }
  };
</script>

<ModalLayout escapable={false}>
  {#if state === State.Welcome}
    <Welcome
      on:next={() => {
        state = State.EnterName;
      }} />
  {:else if state === State.EnterName}
    <EnterName
      {handle}
      on:next={event => {
        handle = event.detail;
        state = State.EnterPassphrase;
      }} />
  {:else if state === State.EnterPassphrase}
    <EnterPassphrase
      on:previous={() => {
        state = State.EnterName;
      }}
      on:next={event => {
        onCreateIdentity(handle, event.detail);
      }} />
  {:else if state === State.SuccessView}
    <Success id={identity.shareableEntityIdentifier} on:close={complete} />
  {/if}
</ModalLayout>
