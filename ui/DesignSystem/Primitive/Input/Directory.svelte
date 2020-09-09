<script>
  import { getDirectoryPath } from "../../../../native/ipc.js";

  import Button from "../Button.svelte";
  import TextInput from "./Text.svelte";

  export let placeholder = null;
  export let style = null;
  export let path = null;
  export let validation = null;
  export let buttonVariant = "primary";

  const openFileDialog = async () => {
    path = await getDirectoryPath();
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
