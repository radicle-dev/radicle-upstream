<script>
  import validatejs from "validate.js";
  import { Button, Flex, Input } from "../../DesignSystem/Primitive";

  import { pop } from "svelte-spa-router";

  export let onNextStep = null;

  const nextStep = () => {
    if (!validatejs.isEmpty(validations)) {
      return;
    }
    onNextStep();
  };

  export let handle = "";
  let validating = false;

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

  let validations = false;
  let timeout = null;

  const validate = async () => {
    validating = true;
    validations = await validatejs({ handle: handle }, constraints);
    if (!validatejs.isEmpty(validations)) {
      validating = false;
    } else {
      // TODO(merle): Use actual avaiability & avatar query
      // TODO(merle): No timeout on load
      clearTimeout(timeout);
      timeout = setTimeout(() => {
        validating = false;
      }, 3000);
    }
  };

  $: validate(handle);
</script>

<Input.Text
  style="--focus-outline-color: var(--color-pink)"
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
    disabled={!handle || validating || validations}
    on:click={nextStep}
    variant="primary">
    Next
  </Button>
</Flex>
