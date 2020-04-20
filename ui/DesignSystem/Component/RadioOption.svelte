<script>
  import { Icon, Title } from "../Primitive";
  import { slide } from "svelte/transition";

  export let title = null;
  export let active = false;
  export let dataCy = null;

  const slotPresent = $$props.$$slots !== undefined;
</script>

<style>
  .option {
    border: 1px solid var(--color-foreground-level-3);
    margin-bottom: 16px;
    border-radius: 4px;
  }

  .option.active {
    box-shadow: 0 0 0 1px var(--color-primary);
    border: 1px solid var(--color-primary);
  }

  .option:hover {
    outline: none;
    box-shadow: 0 0 0 1px var(--color-primary);
    border: 1px solid var(--color-primary);
  }

  .header {
    display: flex;
    justify-content: space-between;
    height: 72px;
    align-items: center;
    padding: 0 24px 0 24px;
    cursor: pointer;
    user-select: none;
  }

  .body {
    border-top: 1px solid var(--color-foreground-level-3);
    background-color: var(--color-foreground-level-1);
    padding: 16px 22px 24px 22px;
    border-radius: 0 0 4px 4px;
  }
</style>

<div class="option" class:active data-cy={dataCy}>
  <div class="header" on:click>
    <Title style="color: var(--color-foreground-level-6)">{title}</Title>
    <Icon.CheckCircle
      style={active ? 'display: block; fill: var(--color-primary)' : 'display: none'} />
  </div>
  {#if slotPresent && active}
    <div class="body" in:slide>
      <slot name="option-body" />
    </div>
  {/if}
</div>
