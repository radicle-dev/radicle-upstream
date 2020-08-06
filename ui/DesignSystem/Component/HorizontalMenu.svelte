<script lang="ts">
  import * as view from "../../src/view";

  import MenuItem from "./HorizontalMenu/MenuItem.svelte";

  export let items: view.MenuItem<string>[];
  export let nav: view.Navigation<string>;

  const current = nav.current;
</script>

<style>
  .menu-list {
    height: 100%;
    align-items: center;
    display: flex;
    flex-direction: row;
  }

  .menu-list-item:first-child {
    margin-left: 16px;
  }
  .menu-list-item {
    margin-left: 24px;
    margin-right: 8px;
    align-items: center;
    line-height: 100%;
  }

  nav :global(.menu-list-item:hover p) {
    color: var(--color-secondary) !important;
  }

  nav :global(.menu-list-item:hover .icon svg) {
    fill: var(--color-secondary);
  }
</style>

<nav data-cy="horizontal-menu">
  <ul class="menu-list">
    {#each items as item}
      <li class="menu-list-item">
        <MenuItem
          icon={item.icon}
          title={item.title}
          on:click={() => nav.set(item.key)}
          active={$current.key === item.key} />
      </li>
    {/each}

  </ul>
</nav>
