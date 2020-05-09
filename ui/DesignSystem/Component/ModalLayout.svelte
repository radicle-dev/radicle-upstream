<script>
  import { pop } from "svelte-spa-router";
  import { Icon } from "../Primitive";

  export let dataCy = null;
  export let full = false;
  export let hideCloseButton = null;
  export let onClose = pop;

  const onKeydown = (event) => {
    if (event.key === "Escape") {
      pop();
    }
  };
</script>

<style>
  .close {
    cursor: pointer;
    position: absolute;
    right: 32px;
    top: 22px;
  }

  .wrapper {
    display: flex;
    justify-content: center;
    width: 100vw;
  }

  .wrapper.center {
    align-items: center;
  }

  .content {
    height: 100%;
    width: 100%;
  }

  .content.center {
    height: auto;
    width: 540px;
  }
</style>

<svelte:window on:keydown={onKeydown} />

{#if !hideCloseButton}
  <div data-cy="modal-close-button" class="close">
    <Icon.CrossBig on:click={onClose} />
  </div>
{/if}

<div class="wrapper" class:center={!full} data-cy={dataCy}>
  <div class="content" class:center={!full}>
    <slot />
  </div>
</div>
