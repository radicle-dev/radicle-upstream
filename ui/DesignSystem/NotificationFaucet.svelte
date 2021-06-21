<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { fade, fly } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { store } from "ui/src/notification";

  import Notification from "./Notification.svelte";

  export let style = "";
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    position: fixed;
    bottom: 2rem;
    z-index: 1001;
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
  }
</style>

<div class="wrapper" {style}>
  {#each $store as notification (notification.id)}
    <div
      data-cy="notification"
      style="max-width: 95%;"
      animate:flip
      in:fly={{ y: 1000, duration: 300 }}
      out:fade>
      <Notification {notification} />
    </div>
  {/each}
</div>
