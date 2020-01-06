<script>
  import Sidebar from "../components/Sidebar.svelte";

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
</style>

<div data-cy={dataCy}>
  <Sidebar />
  <div data-cy="page-container" class="container">
    <slot />
  </div>
</div>
