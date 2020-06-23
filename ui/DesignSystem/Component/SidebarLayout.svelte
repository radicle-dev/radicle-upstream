<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

  import TransactionCenter from "../../App/TransactionCenter.svelte";

  import Sidebar from "./Sidebar.svelte";

  export let dataCy = null;
  export let style = null;

  const session = getContext("session");
</script>

<style>
  .container {
    position: relative;
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
    overflow-x: hidden;
    height: 100%;
  }
</style>

<div data-cy={dataCy}>
  <TransactionCenter />

  <Sidebar
    on:createorg={() => push(path.orgRegistration())}
    identity={session.identity}
    orgs={session.orgs}
    registerOrgPermission={session.permissions.registerOrg} />

  <div class="container" data-cy="scrollable-content">
    <div class="content" {style}>
      <slot />
    </div>
  </div>
</div>
