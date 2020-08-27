<script>
  export let variant = null;
  export let style = null;

  const caption = String(variant);
</script>

<style>
  .badge {
    color: var(--color-secondary);
    background: var(--color-secondary-level-1);
    border-radius: 4px;
    padding: 0.25rem 0.25rem;
    height: 1.5rem;
  }
</style>

<span {style} class="badge typo-text-small">{caption}</span>
