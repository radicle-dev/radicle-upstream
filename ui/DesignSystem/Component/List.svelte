<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let items = null;
</script>

<style>
  ul {
    min-width: 500px;
  }

  li {
    display: flex;
    width: 100%;
    height: 96px;
    flex: 1;
    border-bottom: 1px solid var(--color-foreground-level-3);
    cursor: pointer;
    padding: 22px 15px 26px 12px;
    user-select: none;
  }

  li:hover {
    background-color: var(--color-foreground-level-1);
  }

  li:last-child {
    border-bottom: 0;
  }
</style>

<ul>
  {#each items as item}
    <li
      on:click={() => {
        dispatch('select', item);
      }}
      class="project-card">
      <slot {item} />
    </li>
  {/each}
</ul>
