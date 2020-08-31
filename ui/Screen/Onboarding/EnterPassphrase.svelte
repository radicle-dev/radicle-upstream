<script>
  import { createEventDispatcher } from "svelte";

  import { Button, Input } from "../../DesignSystem/Primitive";

  const dispatch = createEventDispatcher();

  let passphrase;
  let repeatedPassphrase;
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    height: 100%;
  }

  h1 {
    text-align: center;
    width: 11em;
    margin: 0 auto;
    margin-bottom: 1.5rem;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  .buttons {
    display: flex;
    justify-content: flex-end;
    margin-top: 1.5rem;
  }

  .repeat {
    margin: 1.5rem 0 0 0;
  }
</style>

<div class="container">
  <div>
    <h1>Next, you'll enter a passphrase.</h1>

    <p>
      This is used to protect your account on this computer. Think of it like a
      computer’s password. You can’t recover your account with it, but it
      prevents someone from accessing your account if this computer is stolen or
      hacked.
    </p>

    <Input.Password
      dataCy="passphrase-input"
      placeholder="Enter a secure passphrase"
      style="margin-top: 1.5rem;"
      bind:value={passphrase} />

    <div class="repeat" hidden={!passphrase}>
      <p style="color: var(--color-foreground-level-5); margin-bottom: 0.5rem;">
        And enter it again, just to be safe.
      </p>
      <Input.Password
        dataCy="repeat-passphrase-input"
        placeholder="Repeat the secure passphrase"
        bind:value={repeatedPassphrase} />
    </div>

    <div class="buttons">
      <Button
        dataCy="cancel-button"
        variant="transparent"
        style="margin-right: 16px;"
        on:click={() => dispatch('cancel')}>
        Cancel
      </Button>

      <Button
        dataCy="set-passphrase-button"
        disabled={!passphrase || passphrase !== repeatedPassphrase}
        on:click={() => {
          dispatch('next', passphrase);
        }}>
        Set passphrase
      </Button>
    </div>
  </div>
</div>
