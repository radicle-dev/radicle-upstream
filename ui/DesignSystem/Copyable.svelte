<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { SvelteComponent } from "svelte";

  import { copyToClipboard } from "ui/src/ipc";
  import * as notification from "ui/src/notification";
  import Icon from "./Icon";

  export let style = "";
  export let copyContent = "";
  export let notificationText = "Copied to your clipboard";
  export let iconBeforeCopy: typeof SvelteComponent | undefined =
    Icon.CopySmall;
  export let iconAfterCopy: typeof SvelteComponent | undefined =
    Icon.CheckSmall;

  export let styleContent: boolean = true;
  export let showIcon: boolean = true;

  let slotContent: HTMLElement;
  let copyIcon = iconBeforeCopy;

  let copied = false;

  export const copy = (): void => {
    if (copied) {
      return;
    }

    const content = copyContent.length ? copyContent : slotContent.textContent;
    if (content) {
      copyToClipboard(content.trim());
    }

    notification.info({ message: notificationText });

    copied = true;

    copyIcon = Icon.CheckSmall;
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
    max-width: -webkit-fill-available;
  }

  .basic {
    display: flex;
    min-height: 24px;
    width: 100%;
    padding-left: 0.5rem;
  }

  .pad {
    padding-right: 0.5rem;
  }

  .content {
    align-items: center;
    background-color: var(--color-foreground-level-2);
    border-radius: 0.5rem;
    color: var(--color-foreground-level-6);
  }
</style>

<div class="wrapper" on:click|stopPropagation={copy}>
  <span
    class="basic"
    class:content={styleContent}
    class:pad={!showIcon}
    bind:this={slotContent}
    {style}>
    <slot />
    {#if showIcon && iconBeforeCopy && iconAfterCopy}
      <svelte:component
        this={copyIcon}
        style="display: flex; margin-left: 0.25rem; min-width: 24px; margin-right: .5rem;" />
    {/if}
  </span>
</div>
