<script lang="ts">
  import * as screen from "../src/screen";
  import * as session from "../src/session";

  import { Button, Emoji, Input } from "../DesignSystem/Primitive";

  let passphrase = "";
  let unlockInProgress = false;

  const unlock = async () => {
    unlockInProgress = true;
    screen.lock();
    await session.unseal(passphrase).finally(() => {
      screen.unlock();
      unlockInProgress = false;
    });
  };

  const onEnter = () => {
    if (passphrase.length > 0 && !unlockInProgress) {
      unlock();
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
    margin-top: 1.5rem;
  }
</style>

<div class="lock">
  <Emoji emoji={"🚪"} size="huge" style="margin-bottom: 0.75rem;" />
  <h3>Knock, knock</h3>
  <p style="margin-top: 0.5rem; color: var(--color-foreground-level-6);">
    Unlock the app by granting access to your Radicle secret key.
  </p>

  <div class="form">
    <Input.Password
      autofocus
      placeholder="Enter your passphrase"
      bind:value={passphrase}
      disabled={unlockInProgress}
      dataCy="passphrase-input"
      on:enter={onEnter}
      style="width: 16rem; margin-right: 1rem;" />
    <Button
      dataCy="unlock-button"
      disabled={passphrase.length === 0 || unlockInProgress}
      on:click={unlock}>
      Unlock
    </Button>
  </div>
</div>
