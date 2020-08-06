<script>
  import ModalLayout from "../DesignSystem/Component/ModalLayout.svelte";

  const OSKey = navigator.platform.includes("Mac") ? "⌘" : "ctrl";
</script>

<style>
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

<ModalLayout>
  <div class="shortcut">
    <kbd>{OSKey}</kbd>
    +
    <kbd>/</kbd>
    - help
  </div>
  <div class="shortcut">
    <kbd>{OSKey}</kbd>
    +
    <kbd>,</kbd>
    - settings
  </div>
  <div class="shortcut">
    <kbd>{OSKey}</kbd>
    +
    <kbd>d</kbd>
    - design system
  </div>
  <div class="shortcut">
    <kbd>{OSKey}</kbd>
    +
    <kbd>p</kbd>
    - search
  </div>
  <div class="shortcut">
    <kbd>{OSKey}</kbd>
    +
    <kbd>n</kbd>
    - create new project
  </div>
  <div class="shortcut">
    <kbd>esc</kbd>
    - close modal
  </div>
</ModalLayout>
