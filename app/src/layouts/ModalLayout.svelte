<script>
  import { pop } from "svelte-spa-router";
  import { Icon } from "../DesignSystem";
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
  <Icon.CrossBig on:click={pop} />
</div>

<div class="wrapper">
  <div class="content">
    <slot />
  </div>
</div>
