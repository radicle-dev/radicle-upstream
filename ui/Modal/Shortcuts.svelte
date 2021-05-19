<script lang="typescript">
  import { isDev } from "../src/config";

  import * as hotkeys from "../src/hotkeys";
  import { Variant as IllustrationVariant } from "../src/illustration";

  import { Illustration, KeyHint, Modal } from "../DesignSystem/Component";

  const shortcuts = [
    ...hotkeys.shortcuts,
    ...(isDev ? hotkeys.devShortcuts : []),
    hotkeys.escape,
  ];
</script>

<style>
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
  <Illustration
    style="margin-bottom: 1.5rem;"
    variant={IllustrationVariant.Keyboard} />
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
