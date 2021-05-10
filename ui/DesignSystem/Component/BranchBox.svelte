<script lang="typescript">
  import { Icon } from "../Primitive";

  export let name: string;
  export let style: string = "";
</script>

<style>
  .branch {
    display: flex;
    gap: 0.25rem;
    align-items: center;
    padding: 0.5rem;
    background-color: var(--color-foreground-level-2);
    border-radius: 0.25rem;
  }
</style>

<div class="branch" {style}>
  <Icon.Branch />
  <p class="typo-text-bold" style="color: var(--color-foreground-level-6);">
    {name}
  </p>
</div>
