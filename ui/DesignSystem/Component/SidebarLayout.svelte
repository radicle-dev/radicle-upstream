<script>
  import Sidebar from "./Sidebar.svelte";
  import NotificationFaucet from "./NotificationFaucet.svelte";

  export let dataCy = null;
  export let style = null;
</script>

<style>
  .container {
    position: relative;
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
    overflow-x: hidden;
    height: 100%;
  }

  .content {
    margin-top: 64px;
    margin-left: 96px;
    margin-right: 96px;
    margin-bottom: 64px;
  }
</style>

<div data-cy={dataCy}>
  <Sidebar />

  <div class="container">
    <NotificationFaucet style="margin-top: calc(var(--topbar-height) + 11px)" />

    <div class="content" {style}>
      <slot />
    </div>
  </div>
</div>
