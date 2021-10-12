<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { ValidationState } from "ui/src/validation";

  import { createEventDispatcher } from "svelte";
  import validatejs from "validate.js";

  import { ValidationStatus, getValidationState } from "ui/src/validation";

  import { Button, Icon, PasswordInput } from "ui/DesignSystem";

  export let disabled = false;

  const dispatch = createEventDispatcher();

  let passphraseInput: PasswordInput;
  let repeatedPassphraseInput: PasswordInput;
  let passphrase = "";
  let repeatedPassphrase = "";
  let visible: boolean = false;

  let validations: { [key: string]: string[] } | false = false;
  let beginValidation = false;

  // @ts-expect-error: not part of the type definitions
  validatejs.options = {
    fullMessages: false,
  };

  validatejs.validators.optional = (value: unknown, options: unknown) => {
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

  let passphraseValidation: ValidationState = {
    status: ValidationStatus.NotStarted,
  };
  let repeatedPassphraseValidation: ValidationState = {
    status: ValidationStatus.NotStarted,
  };

  // The `dummy` argument allows us to encode reactive dependencies for
  // the caller
  const validate = (_dummy?: unknown) => {
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

    // @ts-expect-error: `validations` is guaranteed to be a validations object
    passphraseValidation = getValidationState("passphrase", validations);
    repeatedPassphraseValidation = getValidationState(
      "repeatedPassphrase",
      // @ts-expect-error: `validations` is guaranteed to be a validations object
      validations
    );
  };

  $: validate([passphrase, repeatedPassphrase]);
  $: if (
    passphrase &&
    repeatedPassphrase &&
    repeatedPassphrase.length >= passphrase.length
  ) {
    beginValidation = true;
    validate();
  }

  $: allowNext =
    passphrase &&
    passphrase === repeatedPassphrase &&
    !validations &&
    !disabled;

  let ran = false;

  const next = () => {
    if (!ran && allowNext) {
      ran = true;
      dispatch("next", passphrase);
    } else {
      repeatedPassphraseInput.focus();
    }
  };

  const onKeydown = (event: KeyboardEvent) => {
    switch (event.code) {
      case "Enter":
        if (passphrase.length === 0) {
          passphraseInput.focus();
          break;
        } else if (repeatedPassphrase.length === 0) {
          repeatedPassphraseInput.focus();
          break;
        } else {
          next();
          break;
        }
    }
  };

  const resetCheck = () => {
    if (passphrase.length === 0) {
      visible = false;
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
    justify-content: space-between;
    margin-top: 1.5rem;
  }

  .back-and-set-buttons {
    display: flex;
    justify-content: flex-end;
  }

  .repeat {
    margin: 1.5rem 0 0 0;
  }
</style>

<svelte:window on:keydown={onKeydown} />

<div class="container" data-cy="enter-passphrase-screen">
  <div>
    <h1>Next, you'll need a passphrase.</h1>

    <p>
      This protects your Radicle keypair in case your device is compromised.
      It's used to sign and publish to the Radicle network. Be sure to store
      this in a safe place —
      <span class="typo-text-bold">
        you’ll need to enter it each time you launch the app!
      </span>
    </p>

    <PasswordInput
      bind:this={passphraseInput}
      on:enter={() => {
        repeatedPassphraseInput.focus();
      }}
      on:change={resetCheck}
      on:keypress={resetCheck}
      autofocus={true}
      dataCy="passphrase-input"
      placeholder="Enter a secure passphrase"
      hint="↵"
      style="margin-top: 1.5rem;"
      validation={passphraseValidation}
      bind:value={passphrase}
      {visible}
      {disabled} />

    <div class="repeat" hidden={!passphrase}>
      <p style="margin-bottom: 0.5rem;">And enter it again, just to be safe.</p>
      <PasswordInput
        bind:this={repeatedPassphraseInput}
        on:enter={next}
        on:change={resetCheck}
        on:keypress={resetCheck}
        dataCy="repeat-passphrase-input"
        placeholder="Repeat the secure passphrase"
        hint="↵"
        validation={repeatedPassphraseValidation}
        bind:value={repeatedPassphrase}
        {visible}
        {disabled} />
    </div>

    <div class="buttons">
      <!-- We’re using backticks for the `variant` property because of
        a bug in Svelte.
        https://github.com/sveltejs/language-tools/issues/1113 -->
      <Button
        dataCy={`${visible ? "hide" : "show"}-passphrase`}
        variant={`transparent`}
        style={`margin-right: 1rem; ${
          passphrase.length === 0 ? "visibility:hidden;" : ""
        }`}
        icon={visible ? Icon.EyeClosed : Icon.EyeOpen}
        on:click={() => (visible = !visible)}>
        {visible ? "Hide" : "Show"} Passphrase
      </Button>
      <div class="back-and-set-buttons">
        <Button
          dataCy="back-button"
          variant="transparent"
          style="margin-right: 1rem;"
          on:click={() => dispatch("previous")}
          {disabled}>
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
</div>
