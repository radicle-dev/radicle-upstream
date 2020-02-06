<script>
  import validatejs from "validate.js";
  import { Button, TextInput } from "../../DesignSystem";

  import { pop } from "svelte-spa-router";

  export let onNextStep = null;

  const nextStep = () => {
    beginValidation = true;
    validate();

    if (validations !== undefined) {
      return;
    }
    onNextStep();
  };

  export let name = "";
  let beginValidation = false;

  validatejs.options = {
    fullMessages: false
  };

  const VALID_NAME_MATCH = new RegExp("^[a-z0-9][a-z0-9_-]+$", "i");
  const constraints = {
    name: {
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

    validations = validatejs({ name: name }, constraints);
  };

  $: validate(name);
</script>

<style>
  .button-row {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    margin-top: 48px;
  }
</style>

<TextInput
  style="--focus-outline-color: var(--color-pink)"
  placeholder="Project name"
  bind:value={name}
  valid={!(beginValidation && validations && validations.name)}
  errorMessage={beginValidation && validations && validations.name && validations.name[0]} />

<div class="button-row">
  <div style="display: flex; flex: 1; align-items: flex-start">
    <Button
      dataCy="cancel-button"
      variant="outline"
      on:click={pop}
      style="margin-right: 24px;">
      Cancel
    </Button>
  </div>
  <Button disabled={!name} on:click={nextStep} variant="primary">Next</Button>
</div>
