<script>
  // TODO(sarah): write tests for this once it's implemented in the ui somewhere
  import { copyToClipboard } from "../../../native/utils.js";

  let slotContent;

  const copy = () => copyToClipboard(slotContent.textContent);
</script>

<style>
  .container {
    cursor: pointer;
  }
</style>

<div class="container" on:click={copy}>
  <span bind:this={slotContent}>
    <slot />
  </span>
</div>
