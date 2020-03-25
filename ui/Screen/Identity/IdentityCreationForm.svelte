<script>
  import { gql } from "apollo-boost";
  import { getClient, mutate } from "svelte-apollo";
  import validatejs from "validate.js";

  import {
    avatarUrlStore,
    avatarFallbackStore,
    displayNameStore,
    handleStore,
    idStore,
    shareableEntityIdentifierStore
  } from "../../store/identity.js";

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
  };

  $: validate(handle, displayName, avatarUrl);

  const client = getClient();

  const CREATE_IDENTITY = gql`
    mutation($handle: String!, $displayName: String, $avatarUrl: String) {
      createIdentity(
        handle: $handle
        displayName: $displayName
        avatarUrl: $avatarUrl
      ) {
        id
        shareableEntityIdentifier
        avatarFallback {
          emoji
          background {
            r
            g
            b
          }
        }
        metadata {
          handle
          displayName
          avatarUrl
        }
      }
    }
  `;

  const createIdentity = async () => {
    let response;

    beginValidation = true;
    validate();

    if (!validatejs.isEmpty(validations)) return;

    try {
      response = await mutate(client, {
        mutation: CREATE_IDENTITY,
        variables: {
          handle: handle,
          displayName: displayName,
          avatarUrl: avatarUrl
        }
      });

      const responseData = response.data.createIdentity;

      idStore.set(responseData.id);
      handleStore.set(responseData.metadata.handle);
      displayNameStore.set(responseData.metadata.displayName);
      avatarUrlStore.set(responseData.metadata.avatarUrl);
      avatarFallbackStore.set(responseData.avatarFallback);
      shareableEntityIdentifierStore.set(
        responseData.shareableEntityIdentifier
      );

      if (onSuccess) onSuccess();
    } catch (error) {
      if (onError) onError(error);
    }
  };
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
    <Text style="margin: 20px 0; color: var(--color-gray);">
      An identity is required to interact on the radicle network. Multiple
      devices can be linked to a single identity.
    </Text>
    <Input.Text
      placeholder="Enter a handle*"
      bind:value={handle}
      dataCy="handle"
      valid={!(validations && validations.handle)}
      validationMessage={validations && validations.handle && validations.handle[0]} />
    <Input.Text
      placeholder="Add a display name"
      bind:value={displayName}
      dataCy="display-name"
      valid={!(validations && validations.displayName)}
      validationMessage={validations && validations.displayName && validations.displayName[0]}
      style="margin-top: 16px;" />
    <Input.Text
      placeholder="Avatar url"
      bind:value={avatarUrl}
      dataCy="avatar-url"
      style="margin: 16px 0 32px 0;"
      validationMessage={validations && validations.avatarUrl && validations.avatarUrl[0]}
      valid={!(validations && validations.avatarUrl)} />
    <div class="buttons">
      <Button
        variant="transparent"
        size="big"
        style="margin-right: 16px;"
        on:click={onCancel}>
        Cancel
      </Button>
      <Button
        dataCy="create-id-button"
        disabled={!handle || validations}
        size="big"
        on:click={createIdentity}>
        Create
      </Button>
    </div>
  </div>
</div>
