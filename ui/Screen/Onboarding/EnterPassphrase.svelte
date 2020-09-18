<script>
  import { createEventDispatcher } from "svelte";
  import validatejs from "validate.js";
  import {
    ValidationStatus,
    getValidationState,
  } from "../../src/validation.ts";

  import { Button, Input } from "../../DesignSystem/Primitive";

  const dispatch = createEventDispatcher();

  let repeatedPassphraseInput;
  let passphrase;
  let repeatedPassphrase;

  let validations = false;
  let beginValidation = false;

  validatejs.options = {
    fullMessages: false,
  };

  validatejs.validators.optional = (value, options) => {
    return !validatejs.isEmpty(value)
      ? validatejs.single(value, options)
      : null;
  };

  const constraints = {
    passphrase: {
      length: {
        minimum: 4,
        message: "Passphrase must be at least 4 characters",
      },
    },
    repeatedPassphrase: {
      equality: {
        message: "Passphrases should match",
        attribute: "passphrase",
      },
    },
  };

  let passphraseValidation = { status: ValidationStatus.NotStarted };
  let repeatedPassphraseValidation = { status: ValidationStatus.NotStarted };

  const validate = () => {
    if (!beginValidation) {
      return;
    }

    validations = validatejs(
      {
        passphrase: passphrase,
        repeatedPassphrase: repeatedPassphrase,
      },
      constraints
    );

    passphraseValidation = getValidationState("passphrase", validations);
    repeatedPassphraseValidation = getValidationState(
      "repeatedPassphrase",
      validations
    );
  };

  $: validate(passphrase, repeatedPassphrase);
  $: if (
    passphrase &&
    repeatedPassphrase &&
    repeatedPassphrase.length >= passphrase.length
  ) {
    beginValidation = true;
    validate();
  }

  $: allowNext =
    passphrase && passphrase === repeatedPassphrase && !validations;

  let ran = false;

  const next = () => {
    if (!ran && allowNext) {
      ran = true;
      dispatch("next", passphrase);
    }
  };
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

<div class="container" data-cy="enter-passphrase-screen">
  <div>
    <h1>Next, you'll need a passphrase.</h1>

    <p>
      This is used to protect your account on this computer. Think of it like a
      computer’s password. <span class="typo-text-bold">
        You can’t recover your account with it,
      </span> but it prevents someone from accessing your account if this computer
      is stolen or hacked.
    </p>

    <Input.Password
      on:enter={() => {
        repeatedPassphraseInput.focus();
      }}
      autofocus={true}
      dataCy="passphrase-input"
      placeholder="Enter a secure passphrase"
      hint="↵"
      style="margin-top: 1.5rem;"
      validation={passphraseValidation}
      bind:value={passphrase} />

    <div class="repeat" hidden={!passphrase}>
      <p style="margin-bottom: 0.5rem;">And enter it again, just to be safe.</p>
      <Input.Password
        bind:inputElement={repeatedPassphraseInput}
        on:enter={next}
        dataCy="repeat-passphrase-input"
        placeholder="Repeat the secure passphrase"
        hint="↵"
        validation={repeatedPassphraseValidation}
        bind:value={repeatedPassphrase} />
    </div>

    <div class="buttons">
      <Button
        dataCy="back-button"
        variant="transparent"
        style="margin-right: 1rem;"
        on:click={() => dispatch('previous')}>
        Back
      </Button>

      <Button
        dataCy="set-passphrase-button"
        disabled={!allowNext}
        on:click={next}>
        Set passphrase
      </Button>
    </div>
  </div>
</div>
