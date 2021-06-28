<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { isDev } from "../src/config";

  import * as hotkeys from "../src/hotkeys";

  import { Emoji, KeyHint, Modal } from "ui/DesignSystem";

  const shortcuts = [
    ...hotkeys.shortcuts,
    ...(isDev ? hotkeys.devShortcuts : []),
    hotkeys.escape,
  ];
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

<Modal dataCy="hotkey-modal">
  <Emoji emoji="⌨️" size="huge" style="margin-bottom: 1rem" />
  <h1>Keyboard shortcuts</h1>
  <div class="shortcuts">
    {#each shortcuts as shortcut}
      <div class="shortcut">
        <KeyHint
          noModifier={!shortcut.modifierKey}
          hint={shortcut.displayKey || shortcut.key} />
        <p class="description">{shortcut.description}</p>
      </div>
    {/each}
  </div>
</Modal>
