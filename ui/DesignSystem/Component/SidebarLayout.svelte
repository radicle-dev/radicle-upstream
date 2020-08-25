<script>
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../../src/path.ts";

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
    height: 100%;
  }
</style>

<div data-cy={dataCy}>
  <Sidebar
    on:createorg={() => push(path.orgRegistration())}
    identity={session.identity}
    orgs={[]} />

  <div class="container">
    <div class="content" {style}>
      <slot />
    </div>
  </div>
</div>
