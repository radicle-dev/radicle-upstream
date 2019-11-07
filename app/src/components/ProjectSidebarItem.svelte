<script>
  import { Icon, Text, Title } from "../DesignSystem";

  export let icon = Icon.Home;
  export let title = "";
  export let active = false;
</script>

<style>
  .icon {
    margin-right: 12px;
    align-self: center;
  }
</style>

{#if active}
  <div class="icon">
    <svelte:component this={icon} style="fill: var(--color-purple)" />
  </div>

  <Title.Regular style="color: var(--color-purple); line-height: 24px">
    {title}
  </Title.Regular>
{:else}
  <div class="icon">
    <svelte:component this={icon} />
  </div>

  <Text.Regular style="color: var(--color-darkgray)">{title}</Text.Regular>
{/if}
