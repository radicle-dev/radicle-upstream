<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let items: any[];

  export let styleHoverState: boolean = true;

  export let dataCy = "";
  export let style = "";
  export let key: string | null = null;
</script>

<style>
  .list-container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
  }

  ul {
    width: 100%;
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
    /* TODO: check if this causes problems in other lists */
    /* changed it to quickly make the checkout drop down visible */
    overflow: visible;
  }

  li {
    display: flex;
    width: 100%;
    flex: 1;
    border-bottom: 1px solid var(--color-foreground-level-2);
    user-select: none;
  }

  .hover:hover {
    background-color: var(--color-foreground-level-1);
    cursor: pointer;
  }

  li:last-child {
    border-bottom: 0;
  }
</style>

<div class="list-container" {style}>
  {#if items.length > 0}
    <ul data-cy={dataCy}>
      {#each items as item, index (key ? item[key] : index)}
        <li
          class:hover={styleHoverState}
          on:click={() => dispatch('select', item)}>
          <slot {item} />
        </li>
      {/each}
    </ul>
  {/if}
</div>
