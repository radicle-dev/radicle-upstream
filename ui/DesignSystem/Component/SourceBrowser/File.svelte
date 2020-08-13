<script>
  import { link } from "svelte-spa-router";

  import { Icon } from "../../Primitive";

  export let name = null;
  export let href = null;
  export let active = false;
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

  a {
    display: flex;
    border-radius: 0.25rem;
  }

  a:hover {
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

<!-- svelte-ignore a11y-missing-attribute -->
<a class="file" class:active use:link={href}>
  <Icon.File />
  <span class="file-name">{name}</span>
</a>
