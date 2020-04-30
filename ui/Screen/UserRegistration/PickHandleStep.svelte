<script>
  import { pop } from "svelte-spa-router";
  import validatejs from "validate.js";

  import * as user from "../../src/user.ts";
  import { showNotification } from "../../store/notification.js";

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
      showNotification({
        text: `Proxy: ${JSON.stringify(error)}`,
        level: "error"
      });
    }
  };

  validatejs.options = {
    fullMessages: false
  };

  const HANDLE_MATCH = "^[a-z0-9][a-z0-9_-]+$";

  const constraints = {
    handle: {
      presence: {
        message: "Handle is required",
        allowEmpty: false
      },
      format: {
        pattern: new RegExp(HANDLE_MATCH),
        message: `Handle should match ${HANDLE_MATCH}`
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
  placeholder="User handle"
  bind:value={handle}
  valid={!(validations && validations.handle)}
  validationMessage={validations && validations.handle && validations.handle[0]}
  variant="handle"
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
