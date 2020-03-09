<script>
  import { pop } from "svelte-spa-router";
  import { Icon } from "../Primitive";

  export let dataCy = null;
  export let onClose = null;
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

<div class="close">
  <Icon.CrossBig on:click={onClose || pop} />
</div>

<div class="wrapper" data-cy={dataCy}>
  <div class="content">
    <slot />
  </div>
</div>
