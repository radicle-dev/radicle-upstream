<script>
  // TODO(sarah): write tests for this once it's implemented in the ui somewhere
  import { copyToClipboard } from "../../../native/ipc.js";
  import * as notification from "../../src/notification.ts";
  import Icon from "../Primitive/Icon";

  export let style = null;
  export let copyContent = null;
  export let notificationText = "Copied to your clipboard";
  export let iconBeforeCopy = Icon.CopySmall;
  export let iconAfterCopy = Icon.Check;
  export let styleContent = true;

  let slotContent;
  let copyIcon = iconBeforeCopy;

  let copied = false;

  const copy = () => {
    if (copied) {
      return;
    }

    const content =
      copyContent !== null ? copyContent : slotContent.textContent;
    copyToClipboard(content.trim());

    notification.info(notificationText);

    copied = true;

    copyIcon = Icon.Check;
    setTimeout(() => {
      copyIcon = Icon.CopySmall;
      copied = false;
    }, 1000);
  };
</script>

<style>
  .wrapper {
    cursor: pointer;
    display: inline-flex;
    white-space: nowrap;
  }

  .content {
    display: flex;
    background-color: var(--color-foreground-level-2);
    padding: 0 4px;
    border-radius: 4px;
    color: var(--color-foreground-level-6);
    align-items: center;
  }
</style>

<div class="wrapper" on:click|stopPropagation={copy}>
  <span class:content={styleContent} bind:this={slotContent} {style}>
    <slot />
    {#if iconBeforeCopy && iconAfterCopy}
      <svelte:component
        this={copyIcon}
        style="display: flex; margin-left: 0.25rem; min-width: 24px;" />
    {/if}
  </span>
</div>
