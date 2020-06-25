<script>
  import Modal from "../Layout/Modal.svelte";
</script>

<style>
  .shortcut {
    margin-bottom: 16px;
  }

  kbd {
    border: 1px solid var(--color-foreground-level-6);
    padding: 4px 8px 4px 8px;
    border-radius: 2px;
  }
</style>

<Modal>
  <div class="shortcut">
    <kbd>SHIFT</kbd>
    +
    <kbd>?</kbd>
    - help
  </div>
  <div class="shortcut">
    <kbd>SHIFT</kbd>
    +
    <kbd>D</kbd>
    - design system
  </div>
  <div class="shortcut">
    <kbd>SHIFT</kbd>
    +
    <kbd>C</kbd>
    - change color theme
  </div>
  <div class="shortcut">
    <kbd>ESC</kbd>
    - close modal
  </div>
</Modal>
