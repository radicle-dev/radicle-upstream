<script>
  export let variant = null;
  export let style = null;

  const caption = String(variant);
</script>

<style>
  .badge {
    color: var(--color-primary);
    background: var(--color-primary-level-1);
    border-radius: 4px;
    padding: 0.125rem 0.25rem;
  }
</style>

<span {style} class="badge typo-text-small">{caption}</span>
