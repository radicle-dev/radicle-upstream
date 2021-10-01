<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  export let variant: "maintainer" | "you" | "default" = "maintainer";
  export let style: string | undefined = undefined;
  export let caption: string | undefined = String(variant);
</script>

<style>
  .badge {
    color: var(--color-primary);
    background: var(--color-primary-level-1);
    border-radius: 0.25rem;
    padding: 0.125rem 0.25rem 0.25rem;
    margin-top: 0.125rem;
    white-space: nowrap;
  }
</style>

<span {style} class="badge typo-text-small">{caption}</span>
