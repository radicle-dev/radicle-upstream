<script lang="typescript">
  import { CSSPosition } from "../../../src/style";

  import Tooltip from "../Tooltip.svelte";

  export let active: boolean = true;
  export let dataCy: string = "";
  export let selected: boolean = false;
  export let style: string = "";
  export let tooltip: string | null = null;
</script>

<style>
  .entry {
    align-items: center;
    background-color: var(--color-background);
    color: var(--color-foreground-level-3);
    cursor: not-allowed;
    display: flex;
    height: 2.5rem;
    justify-content: space-between;
    padding: 0 0.5em;
  }

  .entry.active {
    color: var(--color-foreground-level-6);
  }
  .entry.active:hover {
    background-color: var(--color-foreground-level-2);
    cursor: pointer;
  }

  .entry.selected {
    background-color: var(--color-foreground-level-2);
  }

  .entry :global(p) {
    margin-right: 0.5rem;
    white-space: nowrap;
  }
</style>

{#if tooltip}
  <Tooltip position={CSSPosition.Left} value={tooltip}>
    <div
      class="entry"
      class:active
      class:selected
      data-cy={dataCy}
      on:click|stopPropagation
      {style}>
      <slot />
    </div>
  </Tooltip>
{:else}
  <div
    class="entry"
    class:active
    class:selected
    data-cy={dataCy}
    on:click|stopPropagation
    {style}>
    <slot />
  </div>
{/if}
