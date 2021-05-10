<script lang="typescript">
  import type { SvelteComponent } from "svelte";
  import { createEventDispatcher } from "svelte";

  import type { HorizontalItem } from "../../src/menu";

  import MenuItem from "./HorizontalMenu/MenuItem.svelte";

  export let items: HorizontalItem[];
  export let activeTab: typeof SvelteComponent;
  export let style: string = "";

  const dispatch = createEventDispatcher();

</script>

<style>
  .menu-list {
    height: 100%;
    align-items: center;
    display: flex;
    flex-direction: row;
  }

  .menu-list-item:first-child {
    margin-left: 0px;
  }
  .menu-list-item {
    margin-left: 24px;
    margin-right: 8px;
    align-items: center;
    line-height: 100%;
  }

  nav :global(.menu-list-item:hover p) {
    color: var(--color-primary) !important;
  }

  nav :global(.menu-list-item:hover .icon svg) {
    fill: var(--color-primary);
  }

</style>

<nav data-cy="horizontal-menu" {style}>
  <ul class="menu-list">
    {#each items as item}
      <li class="menu-list-item">
        <MenuItem
          on:click={() => dispatch('select', item)}
          icon={item.icon}
          title={item.title}
          dataCy={`${item.title.toLowerCase()}-tab`}
          counter={item.counter}
          tab={item.tab}
          active={activeTab === item.tab.component} />
      </li>
    {/each}
  </ul>
</nav>
