<script>
  import validatejs from "validate.js";
  import { pop } from "svelte-spa-router";

  import { Input, Text } from "../../Primitive";
  import NavigationButtons from "./NavigationButtons.svelte";

  export let onNextStep = null;
  export let explanatoryText = null;
  export let inputPlaceholder = null;
  export let entity = "Name";

  export let identifier = "";

  let validating = false;
  let validations = false;
  let beginValidation = false;

  const imageUrl =
    "https://pbs.twimg.com/profile_images/378800000411356732/e8b1b7f0bd07d4d948cb2da25e221673_400x400.jpeg";

  const nextStep = () => {
    beginValidation = true;
    validate();

    if (!validatejs.isEmpty(validations)) {
      return;
    }
    onNextStep();
  };

  validatejs.options = {
    fullMessages: false
  };

  const validateAvailability = () => {
    // TODO(sos): implement actual identifier availability
    // validations = { identifier: [`${entity} already taken`] };
  };

  const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");
  const constraints = {
    identifier: {
      presence: {
        message: `${entity} is required`,
        allowEmpty: false
      },
      format: {
        pattern: VALID_NAME_MATCH,
        message: `${entity} should match [a-z0-9][a-z0-9_-]+`
      }
    }
  };

  const validate = () => {
    if (!beginValidation) {
      return;
    }
    validating = true;

    validations = validatejs({ identifier: identifier }, constraints);
    if (!validatejs.isEmpty(validations)) {
      validating = false;
    } else {
      validateAvailability();
      validating = false;
    }
  };

  $: validate(identifier);
</script>

<Text style="color: var(--color-foreground-level-5); margin-bottom: 24px;">
  {explanatoryText}
</Text>
<Input.Text
  style="--focus-outline-color: var(--color-primary); width: 100%;
  margin-bottom: 32px;"
  placeholder={inputPlaceholder}
  {imageUrl}
  variant="avatar"
  bind:value={identifier}
  valid={!(beginValidation && validations && validations.identifier)}
  validationMessage={beginValidation && validations && validations.identifier && validations.identifier[0]}
  validationPending={validating} />

<NavigationButtons onCancel={pop} onNextStep={nextStep} />
