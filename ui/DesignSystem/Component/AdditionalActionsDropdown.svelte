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

  let copyIcon = Icon.Copy;

  const afterCopy = () => {
    copyIcon = Icon.Check;
    setTimeout(() => {
      copyIcon = Icon.Copy;
      hideModal();
    }, 250);
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

  button {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    border-radius: 4px;
    cursor: pointer;
    outline-style: none;
  }

  button :global(svg) {
    fill: var(--color-foreground-level-6);
  }
  button:active :global(svg) {
    fill: var(--color-foreground-level-5);
  }

  button:hover {
    background-color: var(--color-foreground-level-2);
  }

  .modal {
    position: absolute;
    top: 100%;
    right: 0;
    width: 240px;
    margin-top: 15px;
    background-color: var(--color-background);
    box-shadow: 0px 4px 8px var(--color-foreground-level-3-opacity-08);
    border-radius: 4px;
    cursor: pointer;
    border: 1px solid var(--color-foreground-level-3);
  }

  .header {
    padding: 12px 16px;
    border-bottom: solid 1px var(--color-foreground-level-3);
    color: var(--color-foreground-level-5);
    display: flex;
    justify-content: space-between;
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

  .menu-item:first-of-type {
    border-top-left-radius: 2px;
    border-top-right-radius: 2px;
  }

  .menu-item:last-of-type {
    border-bottom-left-radius: 2px;
    border-bottom-right-radius: 2px;
  }

  .menu-item:hover {
    background-color: var(--color-foreground-level-1);
  }
</style>

<svelte:window on:click={hideModal} />
<div data-cy={dataCy} class="container" {style}>
  <button bind:this={triggerEl} on:click|stopPropagation={toggleModal}>
    <svelte:component this={Icon.Ellipses} />
  </button>
  {#if expanded}
    <div out:fade={{ duration: 100 }} class="modal" hidden={!expanded}>
      {#if headerTitle}
        <Copyable {afterCopy}>
          <div class="header">
            <Text
              style="white-space: nowrap; overflow: hidden; text-overflow:
              ellipsis; max-width: 170px;">
              {headerTitle}
            </Text>
            <svelte:component
              this={copyIcon}
              style="margin-left: 8px; min-width: 16px;" />
          </div>
        </Copyable>
      {/if}

      <div class="menu" data-cy="dropdown-menu">
        {#each menuItems as item, index}
          <div
            data-cy={item.dataCy}
            class="menu-item"
            on:click|stopPropagation={handleItemSelection(item)}>
            <svelte:component this={item.icon} style="margin-right: 12px" />
            <Text>{item.title}</Text>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>
