<script>
  import { Button } from "../../Primitive";

  export let onCancel = null;
  export let onNextStep = null;
  export let disabled = null;
  export let nextStepTitle = "Next";

  export let style = null;
</script>

<style>
  .container {
    width: 100%;
    display: flex;
    justify-content: flex-end;
  }
</style>

<div class="container" {style}>
  <div>
    <Button
      dataCy="cancel-button"
      variant="transparent"
      on:click={onCancel}
      style="margin-right: 24px;">
      Cancel
    </Button>
  </div>

  <div>
    <Button {disabled} on:click={onNextStep} variant="primary">
      {nextStepTitle}
    </Button>
  </div>
</div>
