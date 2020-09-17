<script>
  import { isMac } from "./../../src/settings.ts";

  export let hint = null;
  export let modifier = false;
  export let style = null;

  const modifierKey = isMac ? "⌘" : "ctrl";
</script>

<style>
  .hint-container {
    display: flex;
  }

  .hint {
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-5);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
  }

  .hint:first-child {
    margin-right: 0.25rem;
  }
</style>

<div class="hint-container" {style}>
  {#if modifier}
    <p class="typo-text hint">{modifierKey}</p>
  {/if}
  <p class="typo-text hint">{hint}</p>
</div>
