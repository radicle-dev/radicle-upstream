<script>
  import Sidebar from "./Sidebar.svelte";

  let wrapperLeftPadding = $$props.$$slots.nestedSidebar ? "318px" : "68px";
</script>

<style>
  .content {
    margin-top: 80px;
    margin-left: 8vw;
    margin-right: 8vw;
  }
</style>

<div>
  <Sidebar />
  <slot name="nestedSidebar" />

  <div style="width: 100vw; padding-left: {wrapperLeftPadding}">
    <div class="content">
      <slot />
    </div>
  </div>
</div>
