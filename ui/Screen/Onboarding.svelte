<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { fade, fly } from "svelte/transition";

  import { retryFetch } from "ui/src/retryOnError";
  import { State } from "ui/src/onboarding";
  import { createIdentity } from "ui/src/identity";
  import * as error from "ui/src/error";
  import * as router from "ui/src/router";
  import * as screen from "ui/src/screen";
  import * as session from "ui/src/session";

  import Welcome from "./Onboarding/Welcome.svelte";
  import EnterName from "./Onboarding/EnterName.svelte";
  import EnterPassphrase from "./Onboarding/EnterPassphrase.svelte";
  import Success from "./Onboarding/Success.svelte";

  let peerId: string = "";
  let handle: string | undefined;
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

  const goToProfile = () => {
    // App.svelte checks whether to load onboarding or the app depending if
    // the session data is present or not.
    session.fetch();
  };

  const goToWallet = () => {
    // App.svelte checks whether to load onboarding or the app depending if
    // the session data is present or not.
    session.fetch();
    router.push({ type: "wallet", activeTab: "transactions" });
  };

  const onCreateIdentity = async (handle: string, passphrase: string) => {
    await screen.withLock(async () => {
      try {
        createIdentityInProgress = true;
        await session.createKeystore(passphrase);
        // Retry until the API is up
        const identity = await retryFetch(
          () => createIdentity({ handle }),
          100,
          50
        );
        peerId = identity.peerId;
        state = State.SuccessView;
      } catch (err: unknown) {
        animateBackward();
        state = State.EnterName;
        error.show(
          new error.Error({
            code: error.Code.IdentityCreationFailure,
            message: `Could not create identity`,
            details: {
              handle,
            },
            source: err,
          })
        );
      } finally {
        createIdentityInProgress = false;
      }
    });
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
            // @ts-expect-error: the value of `state` guarantees that `handle` is defined
            onCreateIdentity(handle, event.detail);
          }} />
      </div>
    </div>
  {:else if state === State.SuccessView}
    <div class="content" in:fly={{ y: inY }}>
      <div class="inner">
        <Success {peerId} on:profile={goToProfile} on:wallet={goToWallet} />
      </div>
    </div>
  {/if}
</div>
