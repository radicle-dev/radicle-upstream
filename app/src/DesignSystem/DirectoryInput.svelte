<script>
  import { MAIN_IPC_CHANNEL } from "../types.js";
  import Button from "./Button.svelte";
  import Input from "./Input.svelte";
  import Text from "./Text";

  export let placeholder = "";
  export let style = null;
  export let path = "";

  let files;

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
