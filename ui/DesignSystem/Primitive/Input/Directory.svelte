<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { getDirectoryPath } from "../../../src/ipc";

  import type { ButtonVariant } from "../../../src/style";
  import type { ValidationState } from "../../../src/validation";

  import Button from "../Button.svelte";
  import TextInput from "./Text.svelte";

  export let placeholder = "";
  export let style = "";
  export let path = "";
  export let validation: ValidationState | undefined = undefined;
  export let buttonVariant: ButtonVariant = "primary";

  const dispatch = createEventDispatcher();

  const openFileDialog = async () => {
    path = await getDirectoryPath();
    if (path) {
      dispatch("selected");
    }
  };
</script>

<style>
  .wrapper {
    display: flex;
    align-items: flex-start;
  }
</style>

<div class="wrapper" {style}>
  <TextInput
    {placeholder}
    {validation}
    value={path}
    disabled
    style="margin-right: 0.5rem; flex: 1" />

  <Button
    dataCy="choose-path-button"
    variant={buttonVariant}
    on:click={openFileDialog}>
    Choose
  </Button>
</div>
