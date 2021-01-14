<script lang="typescript">
  import { Level } from "../../src/notification";
  import type { Notification } from "../../src/notification";
  import { Icon } from "../Primitive";

  export let notification: Notification;
  export let style = "";

  const icon =
    notification.level === Level.Info
      ? Icon.InfoCircle
      : Icon.ExclamationCircle;
</script>

<style>
  .notification {
    display: flex;
    border-radius: 4px;
    height: 32px;
    align-items: center;
    justify-content: center;
    margin-bottom: 8px;
    user-select: none;
    white-space: nowrap;
  }

  .info {
    color: var(--color-background);
    background-color: var(--color-foreground);
  }

  .info :global(svg) {
    fill: var(--color-background);
  }

  .error {
    color: var(--color-background);
    background-color: var(--color-negative);
  }

  .error :global(svg) {
    fill: var(--color-background);
  }

  .message {
    padding: 0 8px 0 8px;
  }

  .action {
    cursor: pointer;
    margin-right: 8px;
    padding-left: 8px;
    user-select: none;
  }

  .action-divider {
    align-self: stretch;
    width: 1px;
  }

  .error .action-divider {
    background-color: var(--color-negative-level-2);
  }

  .info .action-divider {
    background-color: var(--color-foreground-level-6);
  }
</style>

<div class={`notification ${notification.level.toLowerCase()}`} {style}>
  {#if notification.showIcon}
    <svelte:component this={icon} style="margin-left: 8px; height: 24px" />
  {/if}

  <p class="message typo-overflow-ellipsis">{notification.message}</p>

  {#each notification.actions as action}
    <div class="action-divider" />
    <div class="action typo-text-bold" on:click={action.handler}>
      {action.label}
    </div>
  {/each}
</div>
