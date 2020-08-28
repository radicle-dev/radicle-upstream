<script>
  import { isMac } from "../src/settings.ts";

  const modifierKey = isMac ? "⌘" : "ctrl";
</script>

<style>
  .container {
    width: 616px;
    background: var(--color-background);
    border-radius: 0.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 32px;
  }

  .shortcut {
    margin-bottom: 16px;
    display: flex;
    align-items: center;
  }

  .shortcut:first-child {
    margin-top: 24px;
  }

  kbd {
    border: 1px solid var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
    padding: 8px 12px 8px 12px;
    border-radius: 4px;
  }

  .plus {
    color: var(--color-foreground-level-4);
    margin: 0px 10px;
  }

  .description {
    color: var(--color-foreground-level-6);
    margin: 0px 10px;
  }
</style>

<div class="container">
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
