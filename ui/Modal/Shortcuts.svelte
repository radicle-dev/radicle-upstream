<script>
  import { Illustration } from "../DesignSystem/Component";
  import { isMac } from "../src/settings.ts";
  import { Variant as IllustrationVariant } from "../src/illustration";

  export let content;

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
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-5);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
  }

  .description {
    color: var(--color-foreground-level-6);
    margin: 0rem 0.625rem;
  }

  .container:focus {
    outline: none;
  }
</style>

<div data-cy="hotkey-modal" class="container" bind:this={content}>
  <Illustration
    style="margin-bottom: 1.5rem;"
    variant={IllustrationVariant.Keyboard} />
  <h1>Keyboard shortcuts</h1>
  <div class="shortcuts">
    <div class="shortcut">
      <kbd class="typo-text">?</kbd>
      <p class="description">Keyboard shortcuts</p>
    </div>
    <div class="shortcut">
      <kbd class="typo-text">{modifierKey}+,</kbd>
      <p class="description">Settings</p>
    </div>
    <div class="shortcut">
      <kbd class="typo-text">{modifierKey}+d</kbd>
      <p class="description">Design system</p>
    </div>
    <div class="shortcut">
      <kbd class="typo-text">{modifierKey}+p</kbd>
      <p class="description">Search</p>
    </div>
    <div class="shortcut">
      <kbd class="typo-text">{modifierKey}+n</kbd>
      <p class="description">New project</p>
    </div>
    <div class="shortcut">
      <kbd class="typo-text">esc</kbd>
      <p class="description">Close modal</p>
    </div>
  </div>
</div>
