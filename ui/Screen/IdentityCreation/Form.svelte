<script>
  import { createEventDispatcher } from "svelte";
  import validatejs from "validate.js";

  import { create, store } from "../../src/identity.ts";
  import * as remote from "../../src/remote.ts";
  import {
    ValidationStatus,
    getValidationState,
  } from "../../src/validation.ts";

  import { Button, Input, Text, Title } from "../../DesignSystem/Primitive";

  const dispatch = createEventDispatcher();

  const HANDLE_MATCH = "^[a-z0-9][a-z0-9_-]+$";

  let handle,
    validations,
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

  $: if ($store.status === remote.Status.Success) {
    dispatch("success");
  } else if ($store.status === remote.Status.Error) {
    dispatch("error");
  }
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    height: 100%;
  }

  .buttons {
    display: flex;
    justify-content: flex-end;
  }
</style>

<div class="container">
  <div data-cy="form">
    <Title variant="big" style="text-align: center;">Create an identity</Title>
    <Text style="margin: 20px 0; color: var(--color-foreground-level-5);">
      An identity is required to interact on the radicle network. Multiple
      devices can be linked to a single identity.
    </Text>
    <Input.Text
      placeholder="Enter a handle*"
      bind:value={handle}
      dataCy="handle"
      validation={handleValidation}
      style="margin: 16px 0 32px 0;" />
    <div class="buttons">
      <Button
        dataCy="cancel-button"
        variant="transparent"
        style="margin-right: 16px;"
        on:click={() => dispatch('cancel')}>
        Cancel
      </Button>
      <Button
        dataCy="create-id-button"
        disabled={!handle || validations || $store.status === remote.Status.Loading}
        on:click={() => {
          beginValidation = true;
          validate();
          if (!validatejs.isEmpty(validations)) return;
          create({ handle: handle });
        }}>
        Create
      </Button>
    </div>
  </div>
</div>
