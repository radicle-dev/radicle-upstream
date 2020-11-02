<script lang="ts">
  import * as session from "../src/session";

  import { Button, Emoji, Input } from "../DesignSystem/Primitive";

  let passphrase = "";

  const unlock = async () => {
    const unlocked = await session.unseal(passphrase);
    if (!unlocked) {
      passphrase = "";
    }
  };

  const onEnter = async () => {
    if (passphrase.length > 0) {
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
      dataCy="passphrase-input"
      on:enter={onEnter}
      style="width: 16rem; margin-right: 1rem;" />
    <Button
      dataCy="unlock-button"
      disabled={passphrase.length === 0}
      on:click={unlock}>
      Unlock
    </Button>
  </div>
</div>
