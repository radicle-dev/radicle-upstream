<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { SvelteComponent } from "svelte";
  import type { Notification } from "ui/src/notification";

  import Icon from "./Icon";

  export let notification: Notification;
  export let style: string | undefined = undefined;

  const variantClass = `variant-${notification.variant.toLowerCase()}`;

  let icon: typeof SvelteComponent | null;
  switch (notification.icon) {
    case "InfoCircle":
      icon = Icon.InfoCircle;
      break;
    case "ExclamationCircle":
      icon = Icon.ExclamationCircle;
      break;
    case null:
      icon = null;
      break;
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

  .variant-info {
    background-color: var(--color-foreground);
  }

  .variant-error {
    background-color: var(--color-negative);
  }

  .variant-primary {
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
  }

  .variant-error .action-divider {
    background-color: var(--color-negative-level-2);
  }

  .variant-info .action-divider {
    background-color: var(--color-foreground-level-6);
  }

  .variant-primary .action-divider {
    background-color: var(--color-primary-level-1);
  }
</style>

<div
  class={`notification ${variantClass}`}
  class:bypassLockedScreen={notification.bypassLockedScreen}
  {style}>
  {#if icon}
    <svelte:component this={icon} style="margin-left: 8px; height: 24px" />
  {/if}

  <p class="message typo-overflow-ellipsis">{notification.message}</p>

  {#each notification.actions as action}
    <div class="action-divider" />
    <button
      class="action typo-text-bold"
      data-cy="notification-action"
      on:click={action.handler}>
      {action.label}
    </button>
  {/each}
</div>
