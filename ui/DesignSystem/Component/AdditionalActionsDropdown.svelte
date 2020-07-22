<script>
  import { fade } from "svelte/transition";

  import { Icon, Text } from "../Primitive";
  import Copyable from "./Copyable.svelte";

  let triggerEl;
  let expanded = false;

  const toggleModal = () => {
    expanded = !expanded;
  };

  const hideModal = () => {
    expanded = false;
  };

  const handleItemSelection = item => {
    hideModal();
    item.event();
  };

  export let dataCy = null;
  export let style = null;
  export let menuItems = null;
  export let headerTitle = null;
</script>

<style>
  .container {
    position: relative;
    height: 40px;
    width: 40px;
  }

  .additional-actions-dropdown-button {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 4px;
    cursor: pointer;
    outline-style: none;
  }

  .additional-actions-dropdown-button :global(svg) {
    fill: var(--color-foreground-level-6);
  }

  .additional-actions-dropdown-button:active :global(svg) {
    fill: var(--color-foreground-level-5);
  }

  .additional-actions-dropdown-button:hover {
    background-color: var(--color-foreground-level-2);
  }

  .modal {
    position: absolute;
    top: 100%;
    right: 0;
    width: 240px;
    margin-top: 15px;
    background-color: var(--color-background);
    box-shadow: var(--elevation-medium);
    border-radius: 4px;
    cursor: pointer;
    border: 1px solid var(--color-foreground-level-3);
    overflow: hidden; /* hack to make inner option rounded corners */
    z-index: 1;
    user-select: none;
  }

  .header {
    padding: 12px 16px;
    color: var(--color-foreground-level-5);
    display: flex;
    justify-content: space-between;
    border-bottom: solid 1px var(--color-foreground-level-3);
  }

  .header:hover {
    color: var(--color-foreground-level-6);
  }

  .menu {
    cursor: pointer;
  }

  .menu-item {
    display: flex;
    padding: 8px 12px;
    color: var(--color-foreground-level-6);
  }

  .menu-item:hover {
    background-color: var(--color-foreground-level-1);
  }

  .menu-item.disabled {
    color: var(--color-foreground-level-4);
    cursor: not-allowed;
  }

  .menu-item.disabled :global(svg) {
    fill: var(--color-foreground-level-4);
  }
</style>

<svelte:window on:click={hideModal} />
<div data-cy={dataCy} class="container" {style}>
  <button
    class="additional-actions-dropdown-button"
    bind:this={triggerEl}
    on:click|stopPropagation={toggleModal}>
    <svelte:component this={Icon.Ellipses} />
  </button>
  {#if expanded}
    <div out:fade={{ duration: 100 }} class="modal" hidden={!expanded}>
      {#if headerTitle}
        <div class="header">
          <Copyable iconSize="normal">
            <Text
              style="white-space: nowrap; overflow: hidden; text-overflow:
              ellipsis; max-width: 170px; margin-right: 8px;">
              {headerTitle}
            </Text>
          </Copyable>
        </div>
      {/if}

      {#if menuItems}
        <div class="menu" data-cy="dropdown-menu">
          {#each menuItems as item, index}
            <div
              title={item.tooltip}
              data-cy={item.dataCy}
              class="menu-item"
              class:disabled={item.disabled}
              on:click|stopPropagation={!item.disabled && handleItemSelection(item)}>
              <svelte:component this={item.icon} style="margin-right: 12px" />
              <Text>{item.title}</Text>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
