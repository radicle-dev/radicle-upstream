<script>
  export let href = null;
  export let icon = null;
  export let title = null;
  export let active = null;
</script>

<style>
  .icon {
    margin-right: 12px;
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
    color: var(--color-foreground-level-6);
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
    color: var(--color-secondary);
  }
</style>

<!-- svelte-spa-router link action is not reactive and breaks if the href
     changes dynamically, this is why we have to spell out the href manually -->
<a href={`#${href}`} data-cy={title}>
  {#if active}
    <div class="icon">
      <svelte:component this={icon} style="fill: var(--color-secondary)" />
    </div>
  {:else}
    <div class="icon">
      <svelte:component this={icon} />
    </div>
  {/if}

  <p style={`--title: "${title}"`} class="item" class:active>{title}</p>
</a>

{#if active}
  <slot />
{/if}
