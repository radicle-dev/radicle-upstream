<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import type { HorizontalItem } from "../../src/menu";

  export let items: HorizontalItem[];
  export let activeTab: "files" | "commit" | "commits";
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

  .icon {
    margin-right: 0.5rem;
    align-items: center;
    padding-top: 1px;
  }

  .tab {
    display: flex;
    align-items: center;
    cursor: pointer;
  }

  .title {
    line-height: 130%;
    color: var(--color-foreground-level-6);
  }

  .title.active {
    color: var(--color-primary);
  }

  .counter {
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    padding: 0.1875rem 0.5rem;
    border-radius: 0.75rem;
    margin-left: 0.5rem;
  }

</style>

<nav data-cy="horizontal-menu" {style}>
  <ul class="menu-list">
    {#each items as item}
      <li class="menu-list-item">
        <div
          class="tab"
          data-cy={`${item.title.toLowerCase()}-tab`}
          on:click={() => dispatch('select', item)}>
          <div class="icon">
            <svelte:component
              this={item.icon}
              style={activeTab === item.tab ? 'fill: var(--color-primary)' : ''} />
          </div>

          <p class="title typo-text-bold" class:active={activeTab === item.tab}>
            {item.title}
          </p>
          {#if item.counter}
            <span
              class="counter typo-mono-bold"
              data-cy="counter">{item.counter}</span>
          {/if}
        </div>
      </li>
    {/each}
  </ul>
</nav>
