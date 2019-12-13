<script>
  import Button from "./Button.svelte";
  import Text from "./Text";

  export let placeholder = "";
  export let style = null;

  let input;
  let files;
  let path;

  const openFileDialog = () => {
    input.click();
  };

  const getPath = () => {
    console.log(files);
  };
</script>

<style>
  .wrapper {
    display: flex;
  }

  .path {
    border: 1px solid var(--color-lightgray);
    padding: 8px;
    border-radius: 4px;
    width: 100%;
    height: 48px;
    padding: 0 16px 0 16px;
    text-align: left;
    align-items: center;
    display: flex;
    background-color: var(--color-white);
    margin-right: 16px;
  }
</style>

<div class="wrapper" {style}>
  <div class="path">
    <Text.Regular style="color: var(--color-darkgray)">
      {path || placeholder}
    </Text.Regular>
  </div>

  <input
    bind:this={input}
    on:change={getPath}
    bind:files
    type="file"
    style="display: none"
    webkitdirectory />
  <Button variant="primary" on:click={openFileDialog}>Choose</Button>
</div>
