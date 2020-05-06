<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../lib/path.js";
  import Sidebar from "./Sidebar.svelte";
  import NotificationFaucet from "./NotificationFaucet.svelte";
  import TransactionCenter from "./Transaction/Center.svelte";

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

  .content {
    margin-top: 64px;
    margin-left: 96px;
    margin-right: 96px;
    margin-bottom: 64px;
  }
</style>

<div data-cy={dataCy}>
  <Sidebar
    on:createorg={() => push(path.orgRegistration())}
    orgs={session.orgs}
    identity={session.identity} />

  <div class="container" data-cy="scrollable-content">
    <NotificationFaucet style="margin-top: calc(var(--topbar-height) + 11px)" />

    <div class="content" {style}>
      <slot />
    </div>
  </div>
  <TransactionCenter />
</div>
