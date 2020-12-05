<script lang="ts">
  import type { SvelteComponent } from "svelte";

  import * as notification from "../../src/notification";
  import Icon from "../Primitive/Icon";

  export let style = "";
  export let copyContent = "";
  export let notificationText = "Copied to your clipboard";
  export let iconBeforeCopy: typeof SvelteComponent | undefined =
    Icon.CopySmall;
  export let iconAfterCopy: typeof SvelteComponent | undefined =
    Icon.CheckSmall;

  export let styleContent: boolean = true;
  export let showIcon: boolean = true;

  let slotContent: Element | null = null;
  let textareaElement: HTMLTextAreaElement | null = null;

  let copyIcon = iconBeforeCopy;

  let copied = false;

  const copy = () => {
    if (copied) return;

    const content = copyContent.length ? copyContent : slotContent.textContent;
    if (content) copyToClipboard(content.trim());

    notification.info(notificationText);

    copied = true;

    copyIcon = Icon.CheckSmall;
    setTimeout(() => {
      copyIcon = Icon.CopySmall;
      copied = false;
    }, 1000);
  };

  const copyToClipboard = (text: string) => {
    if (!textareaElement) return;

    textareaElement.value = text;
    textareaElement.select();

    try {
      document.execCommand("copy");
    } catch (error) {
      console.error(error);
    }
  };
</script>

<style>
  .wrapper {
    cursor: pointer;
    display: inline-flex;
    white-space: nowrap;
  }

  .basic {
    display: flex;
    min-height: 24px;
    width: 100%;
    padding-left: 0.25rem;
  }

  .content {
    align-items: center;
    background-color: var(--color-foreground-level-2);
    border-radius: 4px;
    color: var(--color-foreground-level-6);
  }

  textarea {
    position: absolute;
    left: -99999rem;
  }
</style>

<textarea readonly bind:this={textareaElement} />

<div class="wrapper" on:click|stopPropagation={copy}>
  <span
    class="basic"
    class:content={styleContent}
    bind:this={slotContent}
    {style}>
    <slot />
    {#if showIcon && iconBeforeCopy && iconAfterCopy}
      <svelte:component
        this={copyIcon}
        style="display: flex; margin-left: 0.25rem; min-width: 24px;" />
    {/if}
  </span>
</div>
