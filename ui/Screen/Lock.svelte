<script lang="ts">
  import * as session from "../src/session.ts";

  import { Button, Input } from "../DesignSystem/Primitive";

  let passphrase = "";

  const unlock = () => {
    session.unseal(passphrase);
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
</style>

<div class="lock">
  <Input.Text
    autofocus
    placeholder="Enter your passphrase"
    bind:value={passphrase}
    dataCy="passphrase-input"
    style="width: 16rem; margin-bottom: 2rem;" />
  <Button
    dataCy="unlock-button"
    disabled={passphrase.length === 0}
    on:click={unlock}>
    Unlock
  </Button>
</div>
