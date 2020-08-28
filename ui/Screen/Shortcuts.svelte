<script>
  import { Illustration } from "../DesignSystem/Component";
  import { isMac } from "../src/settings.ts";
  import { Variant as IllustrationVariant } from "../src/illustration.ts";

  const modifierKey = isMac ? "⌘" : "ctrl";
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

  kbd {
    border: 0.0625rem solid var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
    padding: 0.5rem 0.75rem 0.5rem 0.75rem;
    border-radius: 0.25rem;
  }

  .plus {
    color: var(--color-foreground-level-4);
    margin: 0rem 0.625rem;
  }

  .description {
    color: var(--color-foreground-level-6);
    margin: 0rem 0.625rem;
  }
</style>

<div class="container">
  <Illustration
    style="margin-bottom: 1.5rem;"
    variant={IllustrationVariant.Keyboard} />
  <h1>Keyboard shortcuts</h1>
  <div class="shortcuts">
    <div class="shortcut">
      <kbd class="typo-text-bold">?</kbd>
      <p class="description">Keyboard shortcuts</p>
    </div>
    <div class="shortcut">
      <kbd class="typo-text-bold">{modifierKey}</kbd>
      <p class="plus">+</p>
      <kbd class="typo-text-bold">,</kbd>
      <p class="description">Settings</p>
    </div>
    <div class="shortcut">
      <kbd class="typo-text-bold">{modifierKey}</kbd>
      <p class="plus">+</p>
      <kbd class="typo-text-bold">d</kbd>
      <p class="description">Design system</p>
    </div>
    <div class="shortcut">
      <kbd class="typo-text-bold">{modifierKey}</kbd>
      <p class="plus">+</p>
      <kbd class="typo-text-bold">p</kbd>
      <p class="description">Search</p>
    </div>
    <div class="shortcut">
      <kbd class="typo-text-bold">{modifierKey}</kbd>
      <p class="plus">+</p>
      <kbd class="typo-text-bold">n</kbd>
      <p class="description">New project</p>
    </div>
    <div class="shortcut">
      <kbd class="typo-text-bold">esc</kbd>
      <p class="description">Close modal</p>
    </div>
  </div>
</div>
