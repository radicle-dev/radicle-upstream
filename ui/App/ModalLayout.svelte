<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as modal from "ui/src/modal";

  const clickOutside = () => {
    modal.hide();
  };

  const modalStore = modal.store;
  $: modalLayout = $modalStore;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  $: componentProps = modalLayout?.modalComponentProps as any;
</script>

<style>
  .modal-layout {
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

<div
  class="modal-layout"
  class:hide={modalLayout === null}
  data-cy="modal-layout">
  <div class="overlay" on:click={clickOutside} />
  <div class="content">
    {#if modalLayout !== null}
      <svelte:component this={modalLayout.modalComponent} {...componentProps} />
    {/if}
  </div>
</div>
