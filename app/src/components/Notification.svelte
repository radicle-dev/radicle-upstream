<script>
  import { Icon, Title } from "../DesignSystem";

  export let icon = Icon.Info;
  export let style = null;

  export let level = "info";

  let color = "var(--color-green)";
  let lightcolor = "var(--color-lightgreen-tint-75)";

  if (level === "warning") {
    color = "var(--color-orange)";
    lightcolor = "var(--color-orange-tint-63)";
  } else if (level === "error") {
    color = "var(--color-bordeaux)";
    lightcolor = "var(--color-red-tint-57)";
  }

  $: document.documentElement.style.setProperty("--color", color);
  $: document.documentElement.style.setProperty("--lightcolor", lightcolor);
</script>

<style>
  .notification {
    display: flex;
    flex: 1;
    border: 1px solid var(--color);
    border-radius: 2px;
    color: var(--color);
    height: 40px;
    align-items: center;
    background-color: var(--lightcolor);
  }

  .close {
    cursor: pointer;
    margin: 0 8px 0 auto;
    height: 24px;
    user-select: none;
  }
</style>

<div class="notification {level}" {style}>
  <svelte:component
    this={icon}
    style="margin: 8px; height: 24px; fill: {color}" />

  <Title.Regular>
    <slot />
  </Title.Regular>
  <div class="close" on:click>
    <Icon.Cross style="fill: {color}" />
  </div>
</div>
