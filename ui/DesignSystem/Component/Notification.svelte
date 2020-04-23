<script>
  import { Icon, Text, Title } from "../Primitive";

  export let style = null;
  export let variant = "info"; // info | error
  export let showIcon = false;
</script>

<style>
  .notification {
    display: flex;
    border-radius: 4px;
    height: 32px;
    align-items: center;
  }

  .info {
    color: var(--color-background);
    background-color: var(--color-foreground);
  }

  .notification.info :global(svg) {
    fill: var(--color-background);
  }

  .error {
    color: var(--color-background);
    background-color: var(--color-negative);
  }

  .notification.error :global(svg) {
    fill: var(--color-background);
  }

  .close {
    cursor: pointer;
    margin: 0 8px 0 6px;
    padding: 5px 0 0 8px;
    height: 32px;
    user-select: none;
    border-left: 1px solid;
  }

  .close.error {
    border-color: var(--color-negative-level-2);
  }

  .close.info {
    border-color: var(--color-foreground-level-6);
  }
</style>

<div class={`notification ${variant}`} {style}>
  {#if showIcon}
    {#if variant === 'info'}
      <Icon.Info style="margin-left: 8px; height: 24px" />
    {:else if variant === 'error'}
      <Icon.Important variant="small" style="margin-left: 8px; height: 24px" />
    {/if}
  {/if}

  <Text style="padding-left: 8px;">
    <slot />
  </Text>

  <div class={`close ${variant}`} on:click>
    <Title>Close</Title>
  </div>
</div>
