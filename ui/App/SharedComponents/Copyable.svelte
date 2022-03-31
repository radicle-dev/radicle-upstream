<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Position } from "design-system/Tooltip";

  import { createEventDispatcher } from "svelte";

  import { copyToClipboard } from "ui/src/ipc";
  import * as notification from "ui/src/notification";

  import Tooltip from "design-system/Tooltip.svelte";

  export let style: string | undefined = undefined;

  // The name of the copyable entity. It will be shown in the tooltip on hover
  // as well as the notification after it's copied to clipboard.
  export let name: string | undefined = undefined;

  // The textual value of the contents of the component slot are copied to the
  // clipboard by default, `clipboardContent` can be used to override that.
  export let clipboardContent: string | undefined = undefined;

  export let tooltipStyle: string | undefined = undefined;
  export let tooltipPosition: Position = "top";

  const dispatch = createEventDispatcher();

  let slotContent: HTMLElement;

  $: tooltipTitle = name ? `Copy ${name} to clipboard` : "Copy to clipboard";
  $: tooltipMessage = name
    ? `${name.replace(/^\w/, c => c.toUpperCase())} copied to your clipboard`
    : "Copied to your clipboard";

  export function copy(): void {
    const content = clipboardContent
      ? clipboardContent
      : slotContent.textContent;

    if (!content) {
      console.warn("Copy to clipboard content is empty");
      return;
    }

    copyToClipboard(content.trim());
    notification.show({ type: "info", message: tooltipMessage });
    dispatch("copy");
  }
</script>

<style>
  .wrapper {
    cursor: pointer;
    display: inline-flex;
    white-space: nowrap;
    max-width: -webkit-fill-available;
  }

  .copyable {
    display: flex;
    min-height: 24px;
    width: 100%;
    align-items: center;
  }
</style>

<Tooltip value={tooltipTitle} position={tooltipPosition} style={tooltipStyle}>
  <div class="wrapper" on:click|stopPropagation={copy}>
    <span class="copyable" bind:this={slotContent} {style}>
      <slot />
    </span>
  </div>
</Tooltip>
