<script lang="typescript">
  import { Icon } from "../../Primitive";

  export let active: boolean;
  export let name: string;
</script>

<style>
  .file {
    display: flex;
    cursor: pointer;
    padding: 0.25rem 0.25rem 0.25rem 0.25rem;
    margin: 0.25rem 0;
    color: var(--color-foreground-level-6);
    line-height: 1.5em;
    flex: 1;
    width: 100%;
  }

  .file-name {
    margin-left: 0.25rem;
    white-space: nowrap;
  }

  .file {
    display: flex;
    border-radius: 0.25rem;
  }

  .file:hover {
    background-color: var(--color-foreground-level-1);
  }

  .active {
    color: var(--color-foreground);
    background-color: var(--color-foreground-level-1);
    font-family: var(--typeface-medium);
  }

  .active :global(svg) {
    fill: var(--color-foreground-level-6);
  }
</style>

<div class="file" class:active on:click>
  <Icon.File />
  <span class="file-name">{name}</span>
</div>
