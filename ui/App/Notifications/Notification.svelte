<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as Notification from "ui/src/notification";

  export let notification: Notification.Notification;
  export let style: string | undefined = undefined;

  const timesShownStore = notification.timesShownStore;
  let actionClicked = false;

  function role(): "alert" | "status" {
    switch (notification.type) {
      case "error":
        return "alert";
      case "info":
      case "primary":
        return "status";
    }
  }
</script>

<style>
  .notification {
    display: flex;
    border-radius: 0.5rem;
    height: 32px;
    align-items: center;
    justify-content: center;
    margin-top: 8px;
    user-select: none;
    white-space: nowrap;
    color: var(--color-background);
  }

  .bypassLockedScreen {
    pointer-events: auto;
    cursor: pointer;
  }

  .notification :global(svg) {
    fill: var(--color-background);
  }

  .info {
    background-color: var(--color-foreground);
  }

  .error {
    background-color: var(--color-negative);
  }

  .primary {
    background-color: var(--color-primary);
  }

  .message {
    padding: 0px 10px;
    max-height: 6rem;
    overflow: auto;
  }

  .action {
    align-self: stretch;
    cursor: pointer;
    padding-right: 8px;
    padding-left: 8px;
    user-select: none;
  }

  .action-divider {
    align-self: stretch;
    width: 1px;
    background-color: var(--color-background);
    opacity: 0.5;
  }
</style>

<div
  role={role()}
  on:mouseenter={() => {
    Notification.removeHideTimer(notification);
  }}
  on:mouseleave={() => {
    if (!actionClicked) {
      Notification.attachHideTimer(notification);
    }
  }}
  class="notification"
  class:info={notification.type === "info"}
  class:error={notification.type === "error"}
  class:primary={notification.type === "primary"}
  class:bypassLockedScreen={notification.bypassLockedScreen}
  {style}>
  <p class="message typo-overflow-ellipsis">
    {#if $timesShownStore > 1}<span>{$timesShownStore}× </span>{/if}
    {notification.message}
  </p>

  {#each notification.actions as action}
    <div class="action-divider" />
    <button
      class="action typo-text-bold"
      data-cy="notification-action"
      on:click={() => {
        actionClicked = true;
        action.handler();
      }}>
      {action.label}
    </button>
  {/each}
</div>
