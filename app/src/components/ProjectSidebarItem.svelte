<script>
  import { Text, Title } from "../DesignSystem";
  import { location, link } from "svelte-spa-router";

  export let projectId;
  export let page;
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

<a href={page.path(projectId)} use:link>
  {#if $location.endsWith(page.title.toLowerCase())}
    <div class="icon">
      <svelte:component this={page.icon} style="fill: var(--color-purple)" />
    </div>

    <Title.Regular style="color: var(--color-purple); line-height: 24px">
      {page.title}
    </Title.Regular>
  {:else}
    <div class="icon">
      <svelte:component this={page.icon} />
    </div>

    <Text.Regular style="color: var(--color-darkgray)">
      {page.title}
    </Text.Regular>
  {/if}
</a>

{#if $location.endsWith(page.title.toLowerCase())}
  <slot />
{/if}
