<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { notificationStore } from "ui/src/notification";

  import Notification from "./Notifications/Notification.svelte";

  export let style: string | undefined = undefined;
</script>

<style>
  .notifications {
    display: flex;
    flex-direction: column;
    align-items: center;
    position: fixed;
    bottom: 2rem;
    z-index: 1001;
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
    /* notifications container spans the whole width of the screen. We
       don’t want it to intercept clicks. */
    pointer-events: none;
  }
</style>

<div class="notifications" {style}>
  {#each $notificationStore as notification (notification.id)}
    <div
      data-cy="notification"
      style="max-width: 95%; pointer-events: auto"
      animate:flip
      in:fly={{ y: 1000, duration: 300 }}
      out:fade>
      <Notification {notification} />
    </div>
  {/each}
</div>
