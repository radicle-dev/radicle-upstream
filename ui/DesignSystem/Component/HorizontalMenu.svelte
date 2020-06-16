<script>
  import { location } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import MenuItem from "./HorizontalMenu/MenuItem.svelte";

  export let items = null;
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
    font-family: var(--typeface-medium);
  }

  nav :global(.menu-list-item:hover .icon svg) {
    fill: var(--color-secondary);
  }
</style>

<nav data-cy="horizontal-menu">
  <ul class="menu-list">
    {#each items as item, i}
      <li class="menu-list-item">
        <MenuItem
          icon={item.icon}
          title={item.title}
          href={item.href}
          active={path.active(item.href, $location, item.looseActiveStateMatching)} />
      </li>
    {/each}

  </ul>
</nav>
