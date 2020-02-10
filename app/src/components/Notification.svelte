<script>
  import { Icon, Text } from "../DesignSystem";

  export let icon = Icon.Info;
  export let style = null;

  export let level = "info";
</script>

<style>
  .notification {
    display: flex;
    flex: 1;
    border-radius: 2px;
    height: 40px;
    align-items: center;
  }

  .info {
    border: 1px solid var(--color-green);
    color: var(--color-green);
    background-color: var(--color-lightgreen-tint-75);
  }

  .notification.info :global(svg) {
    fill: var(--color-green);
  }

  .warning {
    border: 1px solid var(--color-orange);
    color: var(--color-orange);
    background-color: var(--color-orange-tint-65);
  }

  .notification.warning :global(svg) {
    fill: var(--color-orange);
  }

  .error {
    border: 1px solid var(--color-bordeaux);
    color: var(--color-bordeaux);
    background-color: var(--color-red-tint-55);
  }

  .notification.error :global(svg) {
    fill: var(--color-bordeaux);
  }

  .close {
    cursor: pointer;
    margin: 0 8px 0 auto;
    height: 24px;
    user-select: none;
  }
</style>

<div class={`notification ${level}`} {style}>
  <svelte:component this={icon} style="margin: 8px; height: 24px" />

  <Text variant="title">
    <slot />
  </Text>

  <div class="close" on:click>
    <Icon.Cross />
  </div>
</div>
