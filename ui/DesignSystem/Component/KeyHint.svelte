<script lang="ts">
  import { OSModifierKey } from "../../src/hotkeys";

  export let hint = "";
  export let noModifier: boolean = false;
  export let style = "";

  const addModifier = hint.length === 1 && !noModifier && hint !== "↵";
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
  {#if addModifier}
    <p class="typo-text hint">{OSModifierKey}</p>
  {/if}
  <p class="typo-text hint">{hint}</p>
</div>
