<script>
  import { location } from "svelte-spa-router";
  import * as path from "../../../lib/path.js";

  import MenuItem from "./MenuItem.svelte";

  export let items = [];
</script>

<style>
  ul {
    height: 100%;
    align-items: center;
    display: flex;
    flex-direction: row;
  }

  li {
    margin-left: 24px;
    margin-right: 8px;
    align-items: center;
    line-height: 100%;
  }

  nav :global(li:hover p) {
    color: var(--color-purple) !important;
    font-family: var(--typeface-medium);
  }

  nav :global(li:hover .icon svg) {
    fill: var(--color-purple);
  }
</style>

<nav data-cy="project-topbar-menu">
  <ul>
    {#each items as item, i}
      <li>
        <MenuItem
          icon={item.icon}
          title={item.title}
          href={item.href}
          active={path.active(item.href, $location, item.looseActiveStateMatching)} />
      </li>
    {/each}

  </ul>
</nav>
