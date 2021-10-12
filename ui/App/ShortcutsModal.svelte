<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as hotkeys from "ui/src/hotkeys";

  import { KeyHint } from "ui/DesignSystem";
  import Modal from "ui/App/ModalLayout/Modal.svelte";

  const shortcuts = [...hotkeys.shortcuts, hotkeys.escape];
</script>

<style>
  .shortcuts {
    margin: 0 auto;
    width: fit-content;
  }
  .shortcut {
    margin-bottom: 1rem;
    display: flex;
    align-items: center;
  }

  .shortcut:first-child {
    margin-top: 1.5rem;
  }

  .description {
    color: var(--color-foreground-level-6);
    margin: 0rem 0.625rem;
  }
</style>

<Modal dataCy="hotkey-modal" emoji="⌨️" title="Keyboard shortcuts">
  <div class="shortcuts">
    {#each shortcuts as shortcut}
      {#if !shortcut.hide}
        <div class="shortcut">
          {#if shortcut.modifierKey}
            <KeyHint style="margin-right: 0.25rem;"
              >{hotkeys.osModifierKey}</KeyHint>
          {/if}
          <KeyHint>
            {shortcut.displayKey || shortcut.key}
          </KeyHint>
          <p class="description">{shortcut.description}</p>
        </div>
      {/if}
    {/each}
  </div>
</Modal>
