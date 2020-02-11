<script>
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
    padding-top: 1px;
  }

  a {
    display: flex;
    align-items: center;
  }

  .item {
    font-family: var(--typeface-regular);
    font-size: 16px;
    line-height: 130%;
    color: var(--color-darkgray);
  }

  .item::after {
    display: block;
    height: 0;
    color: transparent;
    overflow: hidden;
    visibility: hidden;
    content: var(--title);
    font-family: var(--typeface-medium);
  }

  .item.active {
    font-family: var(--typeface-medium);
    color: var(--color-purple);
  }
</style>

<a {href} use:link title={`Project${title}`}>
  {#if active}
    <div class="icon">
      <svelte:component this={icon} style="fill: var(--color-purple)" />
    </div>
  {:else}
    <div class="icon">
      <svelte:component this={icon} />
    </div>
  {/if}

  <div style={`--title: "${title}"`} class="item" class:active>{title}</div>
</a>

{#if active}
  <slot />
{/if}
