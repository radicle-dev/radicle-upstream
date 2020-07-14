<script>
  // TODO(sarah): write tests for this once it's implemented in the ui somewhere
  import { copyToClipboard } from "../../../native/ipc.js";
  import Icon from "../Primitive/Icon";
  export let style = null;

  export let iconBeforeCopy = Icon.Copy;
  export let iconAfterCopy = Icon.Check;
  export let iconSize = "small";

  let slotContent;

  let copyIcon = iconBeforeCopy;

  const copy = () => {
    copyToClipboard(slotContent.textContent.trim());

    copyIcon = Icon.Check;
    setTimeout(() => {
      copyIcon = Icon.Copy;
    }, 1000);
  };
</script>

<style>
  .wrapper {
    cursor: pointer;
    display: flex;
    white-space: nowrap;
    flex-direction: row;
    align-items: center;
  }

  .content {
    display: flex;
  }
</style>

<div class="wrapper" on:click|stopPropagation={copy}>
  <span class="content" bind:this={slotContent} {style}>
    <slot />
    {#if iconBeforeCopy && iconAfterCopy}
      <svelte:component
        this={copyIcon}
        size={iconSize}
        style="display: flex; min-width: 16px;" />
    {/if}
  </span>
</div>
