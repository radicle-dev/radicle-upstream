<script lang="ts">
  import * as session from "../src/session";
  import * as screen from "../src/screen";

  import { Button, Emoji, Input } from "../DesignSystem/Primitive";

  let passphrase = "";
  let requesting = false;

  const unlock = async () => {
    requesting = true;
    screen.lock();
    await session.unseal(passphrase).finally(() => {
      screen.unlock();
      requesting = false;
    });
  };

  const onEnter = () => {
    if (passphrase.length > 0 && !requesting) {
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
  <Emoji emoji={'🚪'} size="huge" style="margin-bottom: 0.75rem;" />
  <h3>Unlock the app</h3>
  <div class="form">
    <Input.Password
      autofocus
      placeholder="Enter your passphrase"
      bind:value={passphrase}
      disabled={requesting}
      dataCy="passphrase-input"
      on:enter={onEnter}
      style="width: 16rem; margin-right: 1rem;" />
    <Button
      dataCy="unlock-button"
      disabled={passphrase.length === 0 || requesting}
      on:click={unlock}>
      Unlock
    </Button>
  </div>
</div>
