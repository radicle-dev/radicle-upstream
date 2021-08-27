<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as modal from "ui/src/modal";

  const clickOutside = () => {
    modal.hide();
  };

  const modalStore = modal.store;
  // Hack to make svelte typecheck in the markup section.
  $: store = $modalStore;
</script>

<style>
  .modal {
    height: 100vh;
    width: 100vw;
    position: fixed;
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: scroll;
  }

  .overlay {
    background-color: black;
    opacity: 0.7;
    height: 100%;
    width: 100%;
    position: fixed;
  }

  .content {
    z-index: 200;
    margin: auto;
  }

  .hide {
    display: none;
  }
</style>

<div class="modal" class:hide={store === null} data-cy="modal">
  <div class="overlay" on:click={clickOutside} />
  <div class="content">
    {#if store !== null}
      <svelte:component
        this={store.modalComponent}
        {...store.modalComponentProps} />
    {/if}
  </div>
</div>
