<script>
  import { getDirectoryPath } from "../../../../native/utils.js";

  import Button from "../Button.svelte";
  import TextInput from "./Text.svelte";

  export let placeholder = null;
  export let style = null;
  export let valid = true;
  export let path = null;
  export let validationMessage = null;

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
    {validationMessage}
    value={path}
    {valid}
    disabled
    style="margin-right: 16px; flex: 1" />

  <Button variant="primary" on:click={openFileDialog}>Choose</Button>
</div>
