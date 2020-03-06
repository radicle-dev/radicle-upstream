<script>
  import validatejs from "validate.js";
  import { Button, Flex, Input } from "../../DesignSystem/Primitive";

  import { pop } from "svelte-spa-router";

  export let onNextStep = null;

  const nextStep = () => {
    beginValidation = true;
    validate();

    if (!validatejs.isEmpty(validations)) {
      return;
    }
    onNextStep();
  };

  export let handle = "";
  let beginValidation = false;

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

  const validate = () => {
    if (!beginValidation) {
      return;
    }

    validations = validatejs({ handle: handle }, constraints);
  };

  $: validate(handle);
</script>

<Input.Text
  style="--focus-outline-color: var(--color-pink)"
  placeholder="User handle"
  bind:value={handle}
  valid={!(beginValidation && validations && validations.handle)}
  validationMessage={beginValidation && validations && validations.handle && validations.handle[0]} />

<Flex style="margin-top: 48px;">
  <div slot="left">
    <Button
      dataCy="cancel-button"
      variant="outline"
      on:click={pop}
      style="margin-right: 24px;">
      Cancel
    </Button>
  </div>

  <div slot="right">
    <Button disabled={!handle} on:click={nextStep} variant="primary">
      Next
    </Button>
  </div>
</Flex>
