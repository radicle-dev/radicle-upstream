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
    height: 100%;
  }

  .content {
    position: absolute;
    margin: 64px 96px 64px 96px;
  }
</style>

<div data-cy={dataCy}>
  <Sidebar />
  <div data-cy="page-container" class="container">
    <div class="content">
      <slot />
    </div>
  </div>
</div>
