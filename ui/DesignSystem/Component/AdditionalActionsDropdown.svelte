<script>
  import ClickOutside from "svelte-click-outside";

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
    height: 36px;
    width: 36px;
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

  button:hover {
    background-color: var(--color-almostwhite);
  }

  .modal {
    position: absolute;
    top: 100%;
    right: 0;
    width: 240px;
    margin-top: 15px;
    background-color: var(--color-white);
    box-shadow: 0px 4px 8px var(--color-lightgray-opacity-08);
    border-radius: 4px;
    cursor: pointer;
    border: 1px solid var(--color-lightgray);
  }

  .header {
    padding: 12px 16px;
    border-bottom: solid 1px var(--color-lightgray);
    color: var(--color-gray);
    display: flex;
    justify-content: space-between;
  }

  .header:hover {
    color: var(--color-darkgray);
  }

  .menu {
    cursor: pointer;
  }

  .menu-item {
    display: flex;
    padding: 8px 12px;
    color: var(--color-darkgray);
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
    background-color: var(--color-almostwhite);
  }
</style>

<div data-cy={dataCy} class="container" {style} on:click|stopPropagation>
  <button bind:this={triggerEl} on:click|stopPropagation={toggleModal}>
    <svelte:component this={Icon.Ellipses} />
  </button>
  <ClickOutside on:clickoutside={hideModal} exclude={[triggerEl]} useWindow>
    {#if expanded}
      <div class="modal" hidden={!expanded}>
        {#if headerTitle}
          <Copyable>
            <div class="header">
              <Text
                style="white-space: nowrap; overflow: hidden; text-overflow:
                ellipsis; max-width: 170px;">
                {headerTitle}
              </Text>
              <Icon.Copy style="margin-left: 8px; min-width: 16px;" />
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
  </ClickOutside>
</div>
