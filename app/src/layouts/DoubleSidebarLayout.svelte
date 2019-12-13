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
  }
</style>

<div data-cy={dataCy}>
  <Sidebar />
  <div class="container">
    <slot />
  </div>
</div>
