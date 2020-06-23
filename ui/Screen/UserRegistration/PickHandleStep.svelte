<script>
  import { pop } from "svelte-spa-router";
  import validatejs from "validate.js";

  import { idValidationStore } from "../../src/id.ts";

  import { ValidationStatus } from "../../src/validation.ts";

  import { Avatar, Button, Flex, Input } from "../../DesignSystem/Primitive";

  export let identity = null;
  export let onNextStep = null;

  const nextStep = () => {
    if (disableSubmit) {
      return;
    }
    onNextStep();
  };

  export let handle = "";

  // Create a new validation store
  const validation = idValidationStore();

  validatejs.options = {
    fullMessages: false,
  };

  $: {
    validation.validate(handle);
  }

  $: disableSubmit = $validation.status !== ValidationStatus.Success;
</script>

<Input.Text
  dataCy="handle"
  style="--focus-outline-color: var(--color-primary)"
  placeholder="User handle"
  bind:value={handle}
  showSuccessCheck
  validation={$validation}>
  <div slot="avatar">
    <Avatar
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
    disabled={disableSubmit}
    on:click={nextStep}
    variant="primary">
    Next
  </Button>
</Flex>
