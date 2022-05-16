<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let count: number | undefined = undefined;
  export let style: string | undefined = undefined;
</script>

<style>
  .counter {
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    padding: 0.1875rem 0.5rem;
    border-radius: 0.75rem;
    line-height: 1rem;
  }
</style>

{#if count}
  <span {style} class="counter typo-mono-bold" data-cy="counter">{count}</span>
{/if}
