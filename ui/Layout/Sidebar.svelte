<script>
  import { push } from "svelte-spa-router";

  import * as path from "../src/path.ts";

  import Sidebar from "../DesignSystem/Component/Sidebar.svelte";

  export let session = null;
</script>

<style>
  .slot {
    height: 100%;
    left: var(--sidebar-width);
    overflow: hidden;
    position: relative;
    width: calc(100vw - var(--sidebar-width));
  }
</style>

<Sidebar
  on:createorg={() => push(path.orgRegistration())}
  orgs={session.orgs}
  identity={session.identity} />

<div class="slot">
  <slot />
</div>
