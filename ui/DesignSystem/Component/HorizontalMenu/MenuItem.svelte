<script lang="typescript">
  import type { SvelteComponent } from "svelte";
  import type { State } from "ui/src/router";

  import { createEventDispatcher } from "svelte";

  export let tab: State;
  export let dataCy: string;
  export let icon: typeof SvelteComponent;
  export let title: string;
  export let active: boolean;
  export let counter: number | undefined;

  const dispatch = createEventDispatcher();

  const click = () => {
    dispatch("click", { tab });
  };

</script>

<style>
  .icon {
    margin-right: 0.5rem;
    align-items: center;
    padding-top: 1px;
  }

  .tab {
    display: flex;
    align-items: center;
    cursor: pointer;
  }

  .item {
    line-height: 130%;
    color: var(--color-foreground-level-6);
  }

  .item.active {
    color: var(--color-primary);
  }

  .counter {
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    padding: 0.1875rem 0.5rem;
    border-radius: 0.75rem;
    margin-left: 0.5rem;
  }

</style>

<div class="tab" data-cy={dataCy} on:click={click}>
  {#if active}
    <div class="icon">
      <svelte:component this={icon} style="fill: var(--color-primary)" />
    </div>
  {:else}
    <div class="icon">
      <svelte:component this={icon} />
    </div>
  {/if}

  <p class="item typo-text-bold" class:active>{title}</p>
  {#if counter}
    <span class="counter typo-mono-bold" data-cy="counter">{counter}</span>
  {/if}
</div>

{#if active}
  <slot />
{/if}
