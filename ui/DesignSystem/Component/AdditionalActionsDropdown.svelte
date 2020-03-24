<script>
  import ClickOutside from "svelte-click-outside";

  import { copyToClipboard } from "../../lib/nativeUtils.js";
  import { Icon, Text } from "../Primitive";

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
    background-color: var(--color-lightgray-tint-10);
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
    padding: 12px;
    border-bottom: solid 1px var(--color-lightgray);
    display: flex;
    cursor: pointer;
    color: var(--color-gray);
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

  .menu-item:hover {
    background-color: var(--color-lightgray-tint-10);
  }
</style>

<div class="container">
  <button bind:this={triggerEl} on:click={toggleModal}>
    <svelte:component this={Icon.Ellipses} />
  </button>
  <ClickOutside on:clickoutside={hideModal} exclude={[triggerEl]} useWindow>
    {#if expanded}
      <div class="modal" hidden={!expanded}>
        <div class="header" on:click={() => copyToClipboard(headerTitle)}>
          <Text>{headerTitle}</Text>
          <svelte:component this={Icon.Copy} style="margin-left: 8px;" />
        </div>
        <div class="menu">
          {#each menuItems as item}
            <div class="menu-item" on:click={handleItemSelection(item)}>
              <svelte:component this={item.icon} style="margin-right: 12px" />
              <Text>{item.title}</Text>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </ClickOutside>

</div>
