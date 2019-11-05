<script>
  import Sidebar from "./Sidebar.svelte";
</script>

<style>
  .wrapper {
    width: 100vw;
    padding-left: 68px;
  }

  .content {
    margin-top: 80px;
    margin-left: 8vw;
    margin-right: 8vw;
  }
</style>

<div>
  <Sidebar />
  <div class="wrapper">
    <div class="content">
      <slot />
    </div>
  </div>
</div>
