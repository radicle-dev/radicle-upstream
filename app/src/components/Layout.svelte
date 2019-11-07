<script>
  import { location } from "svelte-spa-router";
  import Sidebar from "./Sidebar.svelte";
  import ProjectSidebar from "./ProjectSidebar.svelte";

  let wrapperLeftPadding = $$props.$$slots.nestedSidebar ? "318px" : "68px";

  export let params = {};
</script>

<style>
  .wrapper {
    width: 100vw;
  }

  .content {
    margin-top: 80px;
    margin-left: 8vw;
    margin-right: 8vw;
  }
</style>

<div>
  <Sidebar />
  <slot name="nestedSidebar" />

  <div class="wrapper" style="padding-left: {wrapperLeftPadding}">
    <div class="content">
      <slot />
    </div>
  </div>
</div>
