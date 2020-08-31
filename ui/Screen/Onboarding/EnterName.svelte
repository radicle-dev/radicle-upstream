<script>
  import { createEventDispatcher } from "svelte";
  import validatejs from "validate.js";

  import {
    ValidationStatus,
    getValidationState,
  } from "../../src/validation.ts";

  import { Button, Emoji, Input } from "../../DesignSystem/Primitive";

  export let handle = null;

  const dispatch = createEventDispatcher();

  const HANDLE_MATCH = "^[a-z0-9][a-z0-9_-]+$";

  let validations,
    beginValidation = false;

  validatejs.options = {
    fullMessages: false,
  };

  validatejs.validators.optional = (value, options) => {
    return !validatejs.isEmpty(value)
      ? validatejs.single(value, options)
      : null;
  };

  const constraints = {
    handle: {
      presence: {
        message: "You must provide a handle",
        allowEmpty: false,
      },
      format: {
        pattern: new RegExp(HANDLE_MATCH, "i"),
        message: `Handle should match ${HANDLE_MATCH}`,
      },
    },
  };

  let handleValidation = { status: ValidationStatus.NotStarted };

  const validate = () => {
    if (!beginValidation) {
      return;
    }
    validations = validatejs(
      {
        handle: handle,
      },
      constraints
    );

    handleValidation = getValidationState("handle", validations);
  };

  $: validate(handle);

  $: allowNext = handle && !validations;

  const next = () => {
    if (!allowNext) {
      return;
    }

    beginValidation = true;
    validate();
    if (!validatejs.isEmpty(validations)) return;

    dispatch("next", handle);
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
  }

  p {
    margin: 20px 0;
    color: var(--color-foreground-level-5);
  }

  .buttons {
    display: flex;
    justify-content: flex-end;
  }
</style>

<div class="container">
  <div data-cy="form">
    <h1>
      Hey
      <Emoji emoji="👋 " size="big" style="display: inline;" />
      what should we call you?
    </h1>
    <p>
      You’ll need a name to interact on Radicle. This isn’t unique across the
      platform, but it helps others recognize you a little easier. You can
      change it in your profile at any time.
    </p>
    <Input.Text
      autofocus={true}
      placeholder="Enter a name"
      bind:value={handle}
      on:enterKeydown={next}
      dataCy="handle"
      validation={handleValidation}
      style="margin: 16px 0 32px 0;" />
    <div class="buttons">
      <Button dataCy="next-button" disabled={!allowNext} on:click={next}>
        Looks good
      </Button>
    </div>
  </div>
</div>
