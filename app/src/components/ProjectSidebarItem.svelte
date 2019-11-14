<script>
  import { Icon, Text, Title } from "../DesignSystem";
  import { link, location } from "svelte-spa-router";

  export let projectId = null;
  export let href = null;
  export let icon = null;
  export let title = null;
  export let active = false;
</script>

<style>
  .icon {
    margin-right: 12px;
    align-self: center;
  }

  a {
    display: flex;
  }
</style>

<a {href} use:link>
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
</a>

{#if $location.endsWith(title.toLowerCase())}
  <slot />
{/if}
