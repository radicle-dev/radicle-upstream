<script lang="ts">
  import { BadgeType } from "../../src/badge";

  export let variant = BadgeType.Maintainer;
  export let style = "";

  const caption = String(variant);
</script>

<style>
  .badge {
    color: var(--color-secondary);
    background: var(--color-secondary-level-1);
    border-radius: 4px;
    padding: 0.125rem 0.25rem 0.25rem;
    margin-top: 0.125rem;
  }
</style>

<span {style} class="badge typo-text-small">{caption}</span>
