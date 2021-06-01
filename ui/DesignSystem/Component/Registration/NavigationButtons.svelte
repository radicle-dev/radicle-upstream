<script>
  import { createEventDispatcher } from "svelte";
  import { Button } from "../../Primitive";

  export let disableSubmit = null;
  export let submitLabel = "Next";
  export let cancelLabel = "Cancel";

  export let style = null;

  const dispatch = createEventDispatcher();
  const cancel = () => dispatch("cancel");
  const submit = () => dispatch("submit");
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
      on:click={cancel}
      style="margin-right: 24px;">
      {cancelLabel}
    </Button>
  </div>

  <div>
    <Button
      dataCy="submit-button"
      disabled={disableSubmit}
      on:click={submit}
      variant="primary">
      {submitLabel}
    </Button>
  </div>
</div>
