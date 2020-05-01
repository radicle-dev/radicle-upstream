<script>
  import validatejs from "validate.js";

  import { create, store } from "../../src/identity.ts";
  import * as remote from "../../src/remote.ts";
  import {
    ValidationStatus,
    getValidationState
  } from "../../src/validation.ts";

  import { Button, Input, Text, Title } from "../../DesignSystem/Primitive";

  export let onSuccess,
    onCancel,
    onError = null;

  const HANDLE_MATCH = "^[a-z0-9][a-z0-9_-]+$";
  const DISPLAY_NAME_MATCH = "^[a-z0-9 ]+$";

  let handle,
    displayName,
    avatarUrl,
    validations,
    beginValidation = false;

  validatejs.options = {
    fullMessages: false
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
        allowEmpty: false
      },
      format: {
        pattern: new RegExp(HANDLE_MATCH, "i"),
        message: `Handle should match ${HANDLE_MATCH}`
      }
    },
    displayName: {
      optional: {
        format: {
          pattern: new RegExp(DISPLAY_NAME_MATCH, "i"),
          message: `Display name should match ${DISPLAY_NAME_MATCH}`
        }
      }
    },
    avatarUrl: {
      optional: {
        url: {
          schemes: ["http", "https"],
          message: "Not a valid image URL",
          allowLocal: false
        }
      }
    }
  };

  let handleValidation = { status: ValidationStatus.NotStarted };
  let displayNameValidation = { status: ValidationStatus.NotStarted };
  let avatarUrlValidation = { status: ValidationStatus.NotStarted };

  const validate = () => {
    if (!beginValidation) {
      return;
    }
    validations = validatejs(
      {
        handle: handle,
        displayName: displayName,
        avatarUrl: avatarUrl
      },
      constraints
    );

    handleValidation = getValidationState("handle", validations);
    displayNameValidation = getValidationState("displayName", validations);
    avatarUrlValidation = getValidationState("avatarUrl", validations);
  };

  $: validate(handle, displayName, avatarUrl);

  $: if ($store.status === remote.Status.Success) {
    onSuccess();
  } else if ($store.status === remote.Status.Error) {
    onError();
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
      validation={handleValidation} />
    <Input.Text
      placeholder="Add a display name"
      bind:value={displayName}
      dataCy="display-name"
      validation={displayNameValidation}
      style="margin-top: 16px;" />
    <Input.Text
      placeholder="Avatar url"
      bind:value={avatarUrl}
      dataCy="avatar-url"
      style="margin: 16px 0 32px 0;"
      validation={avatarUrlValidation} />
    <div class="buttons">
      <Button
        variant="transparent"
        style="margin-right: 16px;"
        on:click={onCancel}>
        Cancel
      </Button>
      <Button
        dataCy="create-id-button"
        disabled={!handle || validations || $store.status === remote.Status.Loading}
        on:click={() => {
          beginValidation = true;
          validate();
          if (!validatejs.isEmpty(validations)) return;
          create({
            handle: handle,
            displayName: displayName,
            avatarUrl: avatarUrl
          });
        }}>
        Create
      </Button>
    </div>
  </div>
</div>
