<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let text: string | undefined = undefined;
  export let style: string | undefined = undefined;
</script>

<style>
  .badge {
    color: var(--color-primary);
    background: var(--color-primary-level-1);
    border-radius: 0.25rem;
    padding: 0.125rem 0.5rem 0.25rem;
    white-space: nowrap;
  }
</style>

{#if text}
  <span {style} class="badge typo-text-small">{text}</span>
{/if}
