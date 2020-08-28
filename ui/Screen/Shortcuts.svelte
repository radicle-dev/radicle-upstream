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
  }

  .shortcut:first-child {
    margin-top: 6rem;
  }

  kbd {
    border: 1px solid var(--color-foreground-level-3);
    padding: 4px 8px 4px 8px;
    border-radius: 4px;
  }
</style>

<div class="container">
  <h1>Keyboard shortcuts</h1>
  <div class="shortcuts">
    <div class="shortcut">
      <kbd>?</kbd>
      - Keyboard shortcuts
    </div>
    <div class="shortcut">
      <kbd>{modifierKey}</kbd>
      +
      <kbd>,</kbd>
      - settings
    </div>
    <div class="shortcut">
      <kbd>{modifierKey}</kbd>
      +
      <kbd>d</kbd>
      - design system
    </div>
    <div class="shortcut">
      <kbd>{modifierKey}</kbd>
      +
      <kbd>p</kbd>
      - search
    </div>
    <div class="shortcut">
      <kbd>{modifierKey}</kbd>
      +
      <kbd>n</kbd>
      - create new project
    </div>
    <div class="shortcut">
      <kbd>esc</kbd>
      - close modal
    </div>
  </div>
</div>
