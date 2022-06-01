<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { tick as svelteTick } from "svelte";
  import * as screen from "ui/src/screen";
  import * as session from "ui/src/session";
  import * as error from "ui/src/error";
  import * as notification from "ui/src/notification";

  import EyeOpenIcon from "design-system/icons/EyeOpen.svelte";
  import EyeClosedIcon from "design-system/icons/EyeClosed.svelte";

  import Button from "design-system/Button.svelte";
  import Emoji from "design-system/Emoji.svelte";
  import TextInput from "design-system/TextInput.svelte";

  let passphrase = "";
  let unlockInProgress = false;
  let input: TextInput;
  let isPassphraseConcealed: boolean = true;

  let errorNotificationHandle: notification.Handle | undefined;

  const unlock = async () => {
    if (errorNotificationHandle) {
      errorNotificationHandle.remove();
    }
    await screen.withLock(async () => {
      unlockInProgress = true;
      try {
        const unlocked = await session.unseal(passphrase);
        if (!unlocked) {
          passphrase = "";
          // We wait until the component has re-rendered with
          // `unlockInProgress = false` and the input has been enabled
          // again.
          svelteTick().then(() => {
            input && input.focus();
          });
          errorNotificationHandle = notification.show({
            type: "error",
            message: "That’s the wrong passphrase.",
            actions: [{ label: "Dismiss", handler: () => {} }],
            persist: true,
          });
        }
      } catch (err: unknown) {
        errorNotificationHandle = notification.showException(
          new error.Error({
            message: "Failed to unseal session",
            source: err,
          })
        );
      } finally {
        unlockInProgress = false;
      }
    });
  };

  const onEnter = () => {
    if (passphrase.length > 0 && !unlockInProgress) {
      unlock();
    }
  };

  const resetCheck = () => {
    if (passphrase.length === 0) {
      isPassphraseConcealed = true;
    }
  };
</script>

<style>
  .lock {
    height: 100vh;
    width: 100vw;
    position: fixed;
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    display: flex;
    flex-direction: column;
  }
  .form {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    margin-top: 1.5rem;
  }
  .buttons {
    display: flex;
    justify-content: space-between;
    margin-top: 1rem;
  }
</style>

<div class="lock">
  <Emoji emoji={"🚪"} size="huge" style="margin-bottom: 0.75rem;" />
  <h1>Knock, knock</h1>
  <p style="margin-top: 1rem; color: var(--color-foreground-level-6);">
    Unlock the app by granting access to your Radicle secret key.
  </p>

  <div class="form">
    <TextInput
      variant={isPassphraseConcealed ? { type: "password" } : { type: "text" }}
      bind:this={input}
      autofocus
      placeholder="Enter your passphrase"
      bind:value={passphrase}
      disabled={unlockInProgress}
      dataCy="passphrase-input"
      on:keydown={event => {
        if (event.key === "Enter") {
          onEnter();
        }
      }}
      on:change={resetCheck}
      on:keypress={resetCheck}
      style="width: 20rem;" />

    <div class="buttons">
      <Button
        variant="transparent"
        icon={isPassphraseConcealed ? EyeOpenIcon : EyeClosedIcon}
        on:click={() => (isPassphraseConcealed = !isPassphraseConcealed)}>
        {isPassphraseConcealed ? "Show" : "Hide"} Passphrase
      </Button>
      <Button
        dataCy="unlock-button"
        disabled={passphrase.length === 0 || unlockInProgress}
        on:click={unlock}>
        Unlock
      </Button>
    </div>
  </div>
</div>
