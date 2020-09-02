<script>
  import { slide } from "svelte/transition";

  export let title = null;
  export let active = false;
  export let dataCy = null;
</script>

<style>
  .option {
    border: 0.0625rem solid var(--color-foreground-level-3);
    margin-bottom: 1rem;
    border-radius: 0.25rem;
  }

  .option.active {
    background-color: var(--color-foreground-level-1);
  }

  .option:hover {
    outline: none;
    background-color: var(--color-foreground-level-1);
  }

  .header {
    display: flex;
    flex-direction: column;
    justify-content: center;
    height: 4.5rem;
    cursor: pointer;
    user-select: none;
  }

  .body {
    background-color: var(--color-foreground-level-1);
    padding: 0 1rem 1rem 1rem;
    border-radius: 0 0 0.25rem 0.25rem;
  }
</style>

<div class="option" class:active data-cy={dataCy}>
  <div class="header" on:click>
    <p class="typo-text-bold" style="color: var(--color-foreground-level-6)">
      {title}
    </p>
  </div>
  {#if active}
    <div class="body" in:slide>
      <slot name="option-body" />
    </div>
  {/if}
</div>
