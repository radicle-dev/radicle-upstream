<script>
  import { fade, fly } from "svelte/transition";

  import * as notification from "../src/notification.ts";
  import { State } from "../src/onboarding.ts";
  import { createIdentity } from "../src/identity.ts";
  import * as session from "../src/session.ts";
  import * as urn from "../src/urn.ts";

  import Welcome from "./Onboarding/Welcome.svelte";
  import EnterName from "./Onboarding/EnterName.svelte";
  import EnterPassphrase from "./Onboarding/EnterPassphrase.svelte";
  import Success from "./Onboarding/Success.svelte";

  let identity;
  let handle;
  let state = State.Welcome;

  let inY = 1000;
  let outY = -1000;

  const animateForward = () => {
    inY = 1000;
    outY = -1000;
  };

  const animateBackward = () => {
    inY = -1000;
    outY = 1000;
  };

  const complete = () => {
    // App.svelte checks whether to load onboarding or the app depending if
    // the session data is present or not.
    session.fetch();
  };

  const onCreateIdentity = async (handle, passphrase) => {
    try {
      identity = await createIdentity({
        handle: handle,
        passphrase: passphrase,
      });
      state = State.SuccessView;
    } catch (error) {
      animateBackward();
      state = State.EnterName;
      notification.error(
        `Could not create identity: ${urn.shorten(error.message)}`
      );
    }
  };
</script>

<style>
  .modal {
    position: relative;
    width: 100vw;
    height: 100vh;
  }

  .content {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .inner {
    width: 540px;
  }
</style>

<div class="modal">
  {#if state === State.Welcome}
    <div class="content" in:fade out:fly={{ y: outY }}>
      <div class="inner">
        <Welcome
          on:next={() => {
            state = State.EnterName;
          }} />
      </div>
    </div>
  {:else if state === State.EnterName}
    <div class="content" in:fly={{ y: inY }} out:fly={{ y: outY }}>
      <div class="inner">
        <EnterName
          {handle}
          on:next={event => {
            animateForward();
            handle = event.detail;
            state = State.EnterPassphrase;
          }} />
      </div>
    </div>
  {:else if state === State.EnterPassphrase}
    <div class="content" in:fly={{ y: inY }} out:fly={{ y: outY }}>
      <div class="inner">
        <EnterPassphrase
          on:previous={() => {
            animateBackward();
            state = State.EnterName;
          }}
          on:next={event => {
            onCreateIdentity(handle, event.detail);
          }} />
      </div>
    </div>
  {:else if state === State.SuccessView}
    <div class="content" in:fly={{ y: inY }} out:fade>
      <div class="inner">
        <Success id={identity.shareableEntityIdentifier} on:close={complete} />
      </div>
    </div>
  {/if}
</div>
