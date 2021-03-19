<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  interface Option<T> {
    title: string;
    value: T;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type AnyOption = Option<any>;

  // Currently active option value.
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let active: any;
  // The available options.
  export let options: AnyOption[];

  const onClick = (option: AnyOption) => {
    dispatch("select", option.value);
    currentlyActive = option.value;
  };

  $: currentlyActive = active;
</script>

<style>
  .segmented-control {
    display: flex;
    width: fit-content;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.25rem;
  }
  .segmented-control:hover button.active:not(:hover) {
    background: none;
  }
  .segmented-control button {
    cursor: pointer;
    padding: 0.5625rem 1rem;
    border-radius: 0.25rem;
    margin: 0;
    background-color: var(--color-background);
    color: var(--color-foreground-level-6);
  }
  .segmented-control button:focus {
    outline: none;
  }
  .segmented-control button.active {
    background-color: var(--color-foreground-level-1);
    color: var(--color-secondary);
  }
  .segmented-control button:hover {
    background-color: var(--color-foreground-level-1);
  }
  .segmented-control button:active {
    background-color: var(--color-foreground-level-1);
    color: var(--color-secondary-level-2);
  }
</style>

<div class="segmented-control">
  {#each options as option}
    <button
      class="typo-semi-bold"
      class:active={option.value === currentlyActive}
      value={option.value}
      on:click={() => onClick(option)}>
      {option.title}
    </button>
  {/each}
</div>
