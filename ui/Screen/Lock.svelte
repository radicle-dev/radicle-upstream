<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { tick as svelteTick } from "svelte";
  import * as screen from "ui/src/screen";
  import * as session from "ui/src/session";
  import * as error from "ui/src/error";
  import * as notification from "ui/src/notification";

  import { Button, Emoji, Icon, PasswordInput } from "ui/DesignSystem";

  let passphrase = "";
  let unlockInProgress = false;
  let input: PasswordInput;
  let visible: boolean = false;

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
          errorNotificationHandle = notification.error({
            message: "That’s the wrong passphrase.",
            showIcon: true,
            actions: [{ label: "Dismiss", handler: () => {} }],
            persist: true,
          });
        }
      } catch (err: unknown) {
        errorNotificationHandle = error.show(
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
      visible = false;
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
    <PasswordInput
      {visible}
      bind:this={input}
      autofocus
      placeholder="Enter your passphrase"
      bind:value={passphrase}
      disabled={unlockInProgress}
      dataCy="passphrase-input"
      on:enter={onEnter}
      on:change={resetCheck}
      on:keypress={resetCheck}
      style="width: 20rem;" />

    <div class="buttons">
      <Button
        dataCy={`${visible ? "hide" : "show"}-passphrase`}
        variant="transparent"
        icon={visible ? Icon.EyeClosed : Icon.EyeOpen}
        on:click={() => (visible = !visible)}>
        {visible ? "Hide" : "Show"} Passphrase
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
