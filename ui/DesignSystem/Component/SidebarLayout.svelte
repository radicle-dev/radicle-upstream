<script lang="typescript">
  import { getContext } from "svelte";

  import type { Authenticated } from "../../src/session";

  import Sidebar from "./Sidebar.svelte";

  export let dataCy = "";
  export let style = "";

  const session: Authenticated = getContext("session");
</script>

<style>
  .container {
    position: relative;
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
    height: 100%;
    min-width: 60rem;
  }
</style>

<div data-cy={dataCy}>
  <Sidebar identity={session.identity} />

  <div class="container">
    <div class="content" {style}>
      <slot />
    </div>
  </div>
</div>
