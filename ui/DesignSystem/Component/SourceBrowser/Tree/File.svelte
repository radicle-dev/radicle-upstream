<script lang="typescript">
  import { Icon } from "../../../Primitive";
  import Spinner from "../../../Component/Spinner.svelte";

  export let active: boolean;
  export let dataCy: string = "";
  export let loading: boolean;
  export let name: string;
</script>

<style>
  .file {
    border-radius: 0.5rem;
    color: var(--color-foreground-level-6);
    cursor: pointer;
    display: flex;
    flex: 1;
    line-height: 1.5em;
    margin: 0.25rem 0;
    padding: 0.25rem 0.25rem 0.25rem 0.25rem;
    width: 100%;
  }

  .file:hover {
    background-color: var(--color-foreground-level-1);
  }

  .spinner {
    align-items: center;
    display: flex;
    justify-content: center;
    height: 24px;
    width: 24px;
  }

  .name {
    margin-left: 0.25rem;
    user-select: none;
    white-space: nowrap;
  }

  .active {
    background-color: var(--color-foreground-level-1);
    color: var(--color-foreground);
    font-family: var(--typeface-medium);
  }

  .active :global(svg) {
    fill: var(--color-foreground-level-6);
  }
</style>

<div class="file" class:active on:click data-cy={dataCy}>
  {#if loading}
    <div class="spinner">
      <Spinner height={20} width={20} />
    </div>
  {:else}
    <Icon.File />
  {/if}
  <span class="name">{name}</span>
</div>
