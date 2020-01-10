<script>
  import Sidebar from "../components/Sidebar.svelte";
  import NotificationFaucet from "../components/NotificationFaucet.svelte";

  export let dataCy = null;
</script>

<style>
  .container {
    position: relative;
    left: var(--slim-sidebar-width);
    width: calc(100vw - var(--slim-sidebar-width));
    overflow-x: hidden;
    height: 100%;
  }

  .content {
    margin-top: 64px;
    margin-left: 8vw;
    margin-right: 8vw;
  }
</style>

<div data-cy={dataCy}>
  <Sidebar />
  <div class="container">
    <div class="content">
      <slot />
    </div>
  </div>
</div>
