<script>
  import Sidebar from "./Sidebar.svelte";
</script>

<style>
  .container {
    display: grid;
    grid-template-columns: 68px 1fr;
    grid-template-rows: 1fr;
    grid-column-gap: 0px;
    grid-row-gap: 0px;
  }
  .content {
    padding: 24px;
  }
</style>

<div class="container">
  <Sidebar />
  <div class="content">
    <slot />
  </div>
</div>
