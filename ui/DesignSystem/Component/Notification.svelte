<script>
  import { createEventDispatcher } from "svelte";

  import { Level } from "../../src/notification.ts";

  import { Icon } from "../Primitive";

  const dispatch = createEventDispatcher();

  export let style = null;
  export let level = Level.Info;
  export let showIcon = false;
  export let message = null;
  export let actionText = null;
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
  }

  .info {
    color: var(--color-background);
    background-color: var(--color-foreground);
  }

  .notification.info :global(svg) {
    fill: var(--color-background);
  }

  .error {
    color: var(--color-background);
    background-color: var(--color-negative);
  }

  .notification.error :global(svg) {
    fill: var(--color-background);
  }

  .message {
    padding: 0 8px 0 8px;
  }

  .close {
    cursor: pointer;
    margin-right: 8px;
    padding-left: 8px;
    user-select: none;
    border-left: 1px solid;
  }

  .close.error {
    border-color: var(--color-negative-level-2);
  }

  .close.info {
    border-color: var(--color-foreground-level-6);
  }
</style>

<div class={`notification ${level.toLowerCase()}`} {style}>
  {#if showIcon}
    <svelte:component
      this={level === Level.Info ? Icon.Info : Icon.Important}
      style="margin-left: 8px; height: 24px" />
  {/if}

  <p class="message">{message}</p>

  {#if actionText}
    <div
      class={`close ${level.toLowerCase()}`}
      on:click={() => {
        dispatch('action');
      }}>
      <p class="typo-text-bold">{actionText}</p>
    </div>
  {/if}
</div>
