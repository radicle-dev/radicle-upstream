<script>
  import { fade, fly } from "svelte/transition";

  import { withRetry } from "../src/api.ts";
  import { State } from "../src/onboarding.ts";
  import { createIdentity } from "../src/identity.ts";
  import * as screen from "../src/screen.ts";
  import * as session from "../src/session.ts";
  import * as error from "../src/error";

  import Welcome from "./Onboarding/Welcome.svelte";
  import EnterName from "./Onboarding/EnterName.svelte";
  import EnterPassphrase from "./Onboarding/EnterPassphrase.svelte";
  import Success from "./Onboarding/Success.svelte";

  let identity;
  let handle;
  let state = State.Welcome;
  let createIdentityInProgress = false;

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
      screen.lock();
      createIdentityInProgress = true;
      await session.createKeystore(passphrase);
      // Retry until the API is up
      identity = await withRetry(() => createIdentity({ handle }), 100, 50);
      state = State.SuccessView;
    } catch (err) {
      animateBackward();
      state = State.EnterName;
      error.show({
        code: "create-identity-failed",
        message: `Could not create identity`,
        details: {
          handle,
        },
        source: error.fromException(err),
      });
    } finally {
      screen.unlock();
      createIdentityInProgress = false;
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
    width: 33.75rem;
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
          disabled={createIdentityInProgress}
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
    <div class="content" in:fly={{ y: inY }}>
      <div class="inner">
        <Success peerId={identity.peerId} on:close={complete} />
      </div>
    </div>
  {/if}
</div>
