<script>
  import { pop } from "svelte-spa-router";
  import validatejs from "validate.js";

  import * as user from "../../src/user.ts";

  import { Button, Flex, Input } from "../../DesignSystem/Primitive";

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
      validations = { handle: [error] };
    }
  };

  validatejs.options = {
    fullMessages: false
  };

  const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");
  const constraints = {
    handle: {
      presence: {
        message: "Handle is required",
        allowEmpty: false
      },
      format: {
        pattern: VALID_NAME_MATCH,
        message: "Handle should match [a-z0-9][a-z0-9_-]+"
      }
    }
  };

  const validate = async () => {
    validating = true;
    validations = validatejs({ handle: handle }, constraints);
    if (!validatejs.isEmpty(validations)) {
      validating = false;
    } else {
      await validateHandleAvailability();
      validating = false;
    }
  };

  $: validate(handle);
</script>

<Input.Text
  dataCy="handle"
  avatarFallback={identity.avatarFallback}
  imageUrl={identity.metadata.avatarUrl}
  style="--focus-outline-color: var(--color-primary)"
  placeholder="User handle"
  bind:value={handle}
  valid={!(validations && validations.handle)}
  validationMessage={validations && validations.handle && validations.handle[0]}
  variant="avatar"
  validationPending={validating} />

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
