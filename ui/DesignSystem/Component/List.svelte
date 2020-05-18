<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let dataCy = null;
  export let items = null;
</script>

<style>
  ul {
    min-width: 500px;
  }

  li {
    display: flex;
    width: 100%;
    flex: 1;
    border-bottom: 1px solid var(--color-foreground-level-3);
    cursor: pointer;
    user-select: none;
  }

  li:hover {
    background-color: var(--color-foreground-level-1);
  }

  li:last-child {
    border-bottom: 0;
  }
</style>

<ul data-cy={dataCy}>
  {#each items as item}
    <li
      on:click={() => {
        dispatch('select', item);
      }}>
      <slot {item} />
    </li>
  {/each}
</ul>
