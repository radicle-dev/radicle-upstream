<script>
  import { link } from "svelte-spa-router";
  export let href = null;
  export let icon = null;
  export let title = null;
  export let active = null;
  export let counter = null;
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
    line-height: 130%;
    color: var(--color-foreground-level-6);
  }

  .item.active {
    color: var(--color-secondary);
  }

  .counter {
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    padding: 0 0.5rem;
    border-radius: 0.75rem;
  }
</style>

<!-- svelte-ignore a11y-missing-attribute -->
<a data-cy={title} use:link={href}>
  {#if active}
    <div class="icon">
      <svelte:component this={icon} style="fill: var(--color-secondary)" />
    </div>
  {:else}
    <div class="icon">
      <svelte:component this={icon} />
    </div>
  {/if}

  <p class="item typo-text-bold" class:active>{title}</p>
  {#if counter}<span class="counter typo-mono-bold">{counter}</span>{/if}
</a>

{#if active}
  <slot />
{/if}
