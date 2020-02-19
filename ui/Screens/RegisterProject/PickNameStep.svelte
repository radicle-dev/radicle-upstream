<script>
  import validatejs from "validate.js";
  import { Button, Flex, Input } from "../../DesignSystem/Primitives";

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

  export let projectName = "";
  let beginValidation = false;

  validatejs.options = {
    fullMessages: false
  };

  const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");
  const constraints = {
    projectName: {
      presence: {
        message: "Project name is required",
        allowEmpty: false
      },
      format: {
        pattern: VALID_NAME_MATCH,
        message: "Project name should match [a-z0-9][a-z0-9_-]+"
      }
    }
  };

  let validations = false;

  const validate = () => {
    if (!beginValidation) {
      return;
    }

    validations = validatejs({ projectName: projectName }, constraints);
  };

  $: validate(projectName);
</script>

<Input.Text
  style="--focus-outline-color: var(--color-pink)"
  placeholder="Project name"
  bind:value={projectName}
  valid={!(beginValidation && validations && validations.projectName)}
  validationMessage={beginValidation && validations && validations.projectName && validations.projectName[0]} />

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
    <Button disabled={!projectName} on:click={nextStep} variant="primary">
      Next
    </Button>
  </div>
</Flex>
