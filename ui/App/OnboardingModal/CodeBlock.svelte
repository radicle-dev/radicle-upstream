<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Copy from "design-system/icons/Copy.svelte";
  import Copyable from "ui/App/SharedComponents/Copyable.svelte";

  export let command: string;
</script>

<style>
  .code-block {
    background-color: var(--color-foreground-level-2);
    border-radius: 8px;
    position: relative;
  }

  .inner {
    padding: 16px;
    white-space: nowrap;
    overflow: scroll;
  }

  .command {
    font-family: var(--typeface-mono-bold);
  }

  .output {
    font-family: var(--typeface-mono-regular);
    color: var(--color-foreground-level-6);
    white-space: nowrap;
  }

  .spacer {
    width: 56px;
    display: inline-block;
  }

  .copy-button {
    position: absolute;
    height: 56px;
    width: 56px;
    right: 0;
    top: 0;
    background-color: var(--color-foreground-level-3);
    border-radius: 0 8px 8px 0;
  }

  .copy-button.with-output {
    border-radius: 0 8px 0 8px;
  }
</style>

<div class="code-block">
  <div class="inner">
    <div class="copy-button" class:with-output={$$slots.output}>
      <Copyable style="margin: 16px" clipboardContent={command} name="command">
        <Copy />
      </Copyable>
    </div>
    <div class="command">
      $ {command} <span class="spacer" />
    </div>
    {#if $$slots.output}
      <div class="output">
        <slot name="output" />
      </div>
    {/if}
  </div>
</div>
