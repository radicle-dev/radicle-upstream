<script lang="typescript">
  import { getContext } from "svelte";

  import { fallback } from "../../src/identity";
  import type { Session } from "../../src/session";

  import Sidebar from "./Sidebar.svelte";

  export let dataCy = "";
  export let style = "";

  const session: Session = getContext("session");
  const identity = session.identity || fallback;
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
  <Sidebar {identity} />

  <div class="container">
    <div class="content" {style}>
      <slot />
    </div>
  </div>
</div>
