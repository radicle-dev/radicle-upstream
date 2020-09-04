<script>
  import * as modal from "../../src/modal";

  export let modalRoutes = {};
  const store = modal.store;

  let content;
  const clickOutside = () => {
    modal.hide();
  };
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
  }

  .hide {
    display: none;
  }
</style>

<div class="modal" class:hide={!$store.show}>
  <div class="overlay" on:click={clickOutside} />
  <div class="content">
    <svelte:component
      this={modalRoutes[$store.route]}
      bind:content
      on:hide={modal.hide} />
  </div>
</div>
