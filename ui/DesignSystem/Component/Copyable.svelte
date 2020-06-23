<script>
  // TODO(sarah): write tests for this once it's implemented in the ui somewhere
  import { copyToClipboard } from "../../../native/ipc.js";
  export let style = null;

  let slotContent;

  const copy = () => {
    copyToClipboard(slotContent.textContent.trim());
    afterCopy();
  };

  export let afterCopy = () => {};
</script>

<style>
  .container {
    cursor: pointer;
  }
</style>

<div class="container" on:click|stopPropagation={copy}>
  <span bind:this={slotContent} {style}>
    <slot />
  </span>
</div>
