<script lang="ts">
  import { isDev } from "../../native/ipc.js";

  import * as hotkeys from "../src/hotkeys";
  import { Variant as IllustrationVariant } from "../src/illustration";

  import { Illustration, KeyHint } from "../DesignSystem/Component";

  const shortcuts = isDev()
    ? [...hotkeys.keyboardShortcuts, ...hotkeys.devShortcuts, hotkeys.escape]
    : [...hotkeys.keyboardShortcuts, hotkeys.escape];
</script>

<style>
  .container {
    width: 38.5rem;
    background: var(--color-background);
    border-radius: 0.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
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

  .container:focus {
    outline: none;
  }
</style>

<div data-cy="hotkey-modal" class="container">
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
</div>
