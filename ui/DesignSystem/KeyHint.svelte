<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let style: string | undefined = undefined;
</script>

<style>
  .hint-container {
    display: flex;
  }

  .hint {
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    padding: 0.125rem 0.375rem;
    border-radius: 0.5rem;
    min-width: 1.75rem;
    text-align: center;
  }
</style>

<div class="hint-container" {style}>
  <p class="typo-text hint"><slot /></p>
</div>
