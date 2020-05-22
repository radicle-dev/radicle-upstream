<script>
  import { pop } from "svelte-spa-router";

  import { Icon } from "../Primitive";

  export let dataCy = null;
  export let full = false;
  export let escapable = true;
  export let onClose = pop;

  const onKeydown = (event) => {
    if (
      escapable &&
      event.target === document.body &&
      event.code === "Escape"
    ) {
      onClose();
    }
  };
</script>

<style>
  .close {
    cursor: pointer;
    position: fixed;
    right: 32px;
    top: 22px;
  }

  .modal {
    align-items: center;
    background: var(--color-background);
    display: flex;
    height: 100vh;
    justify-content: center;
    overflow: auto;
    position: fixed;
    right: 0;
    top: 0;
    width: 100vw;
    z-index: 10000;
  }

  .content {
    overflow: visible;
    height: 100%;
    width: 100%;
  }
  .content.center {
    width: 540px;
  }
</style>

<svelte:window on:keydown={onKeydown} />

<div class="modal" data-cy={dataCy}>
  {#if escapable}
    <div data-cy="modal-close-button" class="close">
      <Icon.Cross size="big" on:click={onClose} />
    </div>
  {/if}

  <div class="content" class:center={!full}>
    <slot />
  </div>
</div>
