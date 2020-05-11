<script>
  import { pop } from "svelte-spa-router";
  import validatejs from "validate.js";

  import * as notification from "../../src/notification.ts";
  import * as user from "../../src/user.ts";
  import {
    getValidationState,
    ValidationStatus,
  } from "../../src/validation.ts";

  import { Avatar, Button, Flex, Input } from "../../DesignSystem/Primitive";

  export let identity = null;
  export let onNextStep = null;

  const nextStep = () => {
    if (!validatejs.isEmpty(validations)) {
      return;
    }
    onNextStep();
  };

  export let handle = "";

  let validating = false;
  let validations = false;

  const validateHandleAvailability = async () => {
    try {
      const present = await user.get(handle);

      if (present) {
        validations = { handle: ["Handle already taken"] };
      }
    } catch (error) {
      notification.error(`Proxy: ${JSON.stringify(error)}`);
    }
  };

  validatejs.options = {
    fullMessages: false,
  };

  const HANDLE_MATCH = "^[a-z0-9][a-z0-9_-]+$";

  const constraints = {
    handle: {
      presence: {
        message: "Handle is required",
        allowEmpty: false,
      },
      format: {
        pattern: new RegExp(HANDLE_MATCH),
        message: `Handle should match ${HANDLE_MATCH}`,
      },
    },
  };

  let handleValidation = { status: ValidationStatus.NotStarted };
  const validate = async () => {
    handleValidation = { status: ValidationStatus.Loading };
    validations = validatejs({ handle: handle }, constraints);
    if (!validatejs.isEmpty(validations)) {
      handleValidation = getValidationState("handle", validations);
    } else {
      await validateHandleAvailability();
      validating = false;
      handleValidation = getValidationState("handle", validations);
    }
  };

  $: validate(handle);
</script>

<Input.Text
  dataCy="handle"
  style="--focus-outline-color: var(--color-primary)"
  placeholder="User handle"
  bind:value={handle}
  showSuccessCheck
  validation={handleValidation}>
  <div slot="avatar">
    <Avatar
      imageUrl={identity.metadata.avatarUrl}
      avatarFallback={identity.avatarFallback}
      size="small"
      variant="circle" />
  </div>
</Input.Text>

<Flex style="margin-top: 32px;" align="right">
  <Button
    dataCy="cancel-button"
    variant="transparent"
    on:click={pop}
    style="margin-right: 24px;">
    Cancel
  </Button>
  <Button
    dataCy="next-button"
    disabled={!handle || validating || validations}
    on:click={nextStep}
    variant="primary">
    Next
  </Button>
</Flex>
