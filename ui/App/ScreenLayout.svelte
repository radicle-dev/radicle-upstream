<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as Session from "ui/src/session";

  import Sidebar from "./Sidebar.svelte";

  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;

  const session = Session.unsealed();
</script>

<style>
  .container {
    position: relative;
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
    min-width: 60rem;
    height: 100%;
  }

  .content {
    padding-bottom: 1rem;
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
