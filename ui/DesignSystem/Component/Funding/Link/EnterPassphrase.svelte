<script lang="typescript">
  import { Button, Input } from "../../../Primitive";
  import { Illustration } from "../../../Component";

  import { Variant as IllustrationVariant } from "../../../../src/illustration";

  export let passphrase: string;
  export let onCancel: () => void;
  export let onConfirm: () => void;
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
  }

  header {
    padding: 0px var(--content-padding);
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: 1.5rem;
  }
</style>

<div class="wrapper">
  <Illustration variant={IllustrationVariant.Socks} />

  <header>
    <h1 style="margin-top: 1.5rem;">Add your Ethereum account to Radicle</h1>
    <p
      style="margin-top: 1.5rem; padding: 0 var(--content-padding)"
      class="typo-text">
      Are you sure you want to add your ethereum address to your Radicle
      account?
    </p>
  </header>

  <Input.Password
    dataCy="passphrase-input"
    bind:value={passphrase}
    placeholder="Enter your passphrase"
    on:enter={() => (passphrase.length > 0 ? onConfirm() : {})}
    autofocus
    style={'width: 60%; margin-top: 1.5rem;'} />

  <div class="submit">
    <Button variant="transparent" dataCy="cancel-topup" on:click={onCancel}>
      Cancel
    </Button>

    <Button
      disabled={passphrase.length === 0}
      dataCy="confirm-button"
      on:click={onConfirm}
      style="margin-left: 14px;">
      Confirm
    </Button>
  </div>
</div>
