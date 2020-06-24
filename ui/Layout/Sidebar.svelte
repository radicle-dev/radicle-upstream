<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../src/path.ts";

  import Sidebar from "../DesignSystem/Component/Sidebar.svelte";

  export let dataCy = null;
  export let style = null;

  const session = getContext("session");
</script>

<style>
  .layout {
    position: relative;
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
    overflow-x: hidden;
    height: 100%;
  }
</style>

<div data-cy={dataCy}>
  <Sidebar
    on:createorg={() => push(path.orgRegistration())}
    identity={session.identity}
    orgs={session.orgs}
    registerOrgPermission={session.permissions.registerOrg} />

  <div class="layout" data-cy="scrollable-content">
    <div class="content" {style}>
      <slot />
    </div>
  </div>
</div>
