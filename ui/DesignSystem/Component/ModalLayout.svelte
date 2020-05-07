<script>
  import { pop } from "svelte-spa-router";
  import { Icon } from "../Primitive";
  import NotificationFaucet from "./NotificationFaucet.svelte";

  export let dataCy = null;
  export let hideCloseButton = null;
  export let onClose = pop;
</script>

<style>
  .close {
    position: absolute;
    top: 22px;
    right: 32px;
    cursor: pointer;
  }

  .wrapper {
    display: flex;
    justify-content: center;
    width: 100vw;
  }

  .content {
    width: 540px;
  }
</style>

{#if !hideCloseButton}
  <div data-cy="modal-close-button" class="close">
    <Icon.CrossBig on:click={onClose} />
  </div>
{/if}

<div class="wrapper" data-cy={dataCy}>
  <NotificationFaucet style="margin-top: calc(var(--topbar-height) + 11px)" />
  <div class="content">
    <slot />
  </div>
</div>
