<script>
  import { Icon, Text, Title } from "../DesignSystem";
  import { link } from "svelte-spa-router";

  export let href = null;
  export let icon = null;
  export let title = null;
  export let active = null;
</script>

<style>
  .icon {
    margin-right: 16px;
    align-items: center;
  }

  a {
    display: flex;
    align-items: center;
  }
</style>

<a {href} use:link title={`Project${title}`}>
  {#if active}
    <div class="icon">
      <svelte:component this={icon} style="fill: var(--color-purple)" />
    </div>

    <Title.Regular style="color: var(--color-purple)">{title}</Title.Regular>
  {:else}
    <div class="icon">
      <svelte:component this={icon} />
    </div>

    <Text.Regular style="color: var(--color-darkgray)">{title}</Text.Regular>
  {/if}
</a>

{#if active}
  <slot />
{/if}
