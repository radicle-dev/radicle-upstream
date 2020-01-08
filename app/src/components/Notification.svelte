<script>
  import { Icon, Title } from "../DesignSystem";

  export let icon = Icon.Info;
  export let style = null;
</script>

<style>
  .notification {
    display: flex;
    flex: 1;
    border: 1px solid var(--color-green);
    border-radius: 2px;
    color: var(--color-green);
    height: 40px;
    align-items: center;
    background-color: var(--color-lightgreen-tint-75);
  }

  .close {
    cursor: pointer;
    margin: 0 8px 0 auto;
    height: 24px;
    user-select: none;
  }
</style>

<div class="notification" {style}>
  <svelte:component
    this={icon}
    style="margin: 8px; height: 24px; fill: var(--color-green)" />

  <Title.Regular>
    <slot />
  </Title.Regular>
  <div class="close" on:click>
    <Icon.Cross style="fill: var(--color-green)" />
  </div>
</div>
