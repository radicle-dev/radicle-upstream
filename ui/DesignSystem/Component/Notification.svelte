<script>
  import { Icon, Title } from "../Primitive";

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
    border: 1px solid var(--color-positive);
    color: var(--color-positive);
    background-color: var(--color-positive-level-1);
  }

  .notification.info :global(svg) {
    fill: var(--color-positive);
  }

  .warning {
    border: 1px solid var(--color-caution);
    color: var(--color-caution);
    background-color: var(--color-caution-level-1);
  }

  .notification.warning :global(svg) {
    fill: var(--color-caution);
  }

  .error {
    border: 1px solid var(--color-negative-level-6);
    color: var(--color-negative-level-6);
    background-color: var(--color-negative-level-1);
  }

  .notification.error :global(svg) {
    fill: var(--color-negative-level-6);
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

  <Title>
    <slot />
  </Title>

  <div class="close" on:click>
    <Icon.Cross />
  </div>
</div>
