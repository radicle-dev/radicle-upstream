<script>
  import { MAIN_IPC_CHANNEL } from "../types.js";
  import Button from "./Button.svelte";
  import Input from "./Input.svelte";
  import Text from "./Text";

  export let placeholder = "";
  export let style = null;
  export let path = "";

  let files;

  // We have to be able to select empty directories when we create new
  // projects. Unfortunately we can't use the HTML5 open dialog via
  // <input type="file"> for this. Although it lets us select directories,
  // it doesn't fire an event when an empty directory is selected.
  //
  // The workaround is to use the electron native open dialog. As a bonus we
  // can configure it to allow users to create new directories.
  const openFileDialog = async () => {
    path = await window.electron.ipcRenderer.invoke(MAIN_IPC_CHANNEL);
  };
</script>

<style>
  .wrapper {
    display: flex;
  }
</style>

<div class="wrapper" {style}>
  <Input
    {placeholder}
    value={path || placeholder}
    disabled
    style="margin-right: 16px" />

  <Button variant="primary" on:click={openFileDialog}>Choose</Button>
</div>
