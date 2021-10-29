<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let title: string;
</script>

<style>
  .swatch {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    padding: 2rem 0;
    border-top: 1px solid var(--color-foreground-level-2);
  }

  p {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="swatch">
  <p class="typo-text-small">{title}</p>
  <slot />
</div>
