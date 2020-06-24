<script>
  import { Icon } from "../../Primitive";

  export let name = null;
  export let active = false;
</script>

<style>
  .file {
    display: flex;
    cursor: pointer;
    padding: 4px 4px 4px 4px;
    margin: 4px 0;
    color: var(--color-foreground-level-6);
    line-height: 1.5em;
    flex: 1;
    width: 100%;
    display: flex;
    border-radius: 4px;
    white-space: nowrap;
    user-select: none;
  }

  .file:hover {
    background-color: var(--color-foreground-level-1);
  }

  /* prevent icon from shrinking when the filename is long */
  .file :global(svg) {
    overflow: unset;
  }

  .file-name {
    margin-left: 0.25rem;
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
