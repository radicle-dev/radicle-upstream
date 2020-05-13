<script>
  // Currently active option.
  export let active = null;
  // The available options.
  export let options = null;

  const onClick = (option) => {
    option.event();
    currentlyActive = option.value;
  };

  $: currentlyActive = active;
</script>

<style>
  .segmented-control {
    display: flex;
  }
  .segmented-control button {
    cursor: pointer;
    padding: 8px 16px;
    border-radius: 4px;
    margin: 0;
    background-color: var(--color-background);
    color: var(--color-foreground-level-6);
    font-family: var(--typeface-medium);
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
      class:active={option.value === currentlyActive}
      value={option.value}
      on:click={() => onClick(option)}>
      {option.title}
    </button>
  {/each}
</div>
