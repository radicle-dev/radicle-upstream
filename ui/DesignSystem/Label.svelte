<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  export let title: string;
  export let color: string;
  export let style: string = "";
</script>

<style>
  .label {
    background-color: var(--color);
    padding: 2px 6px;
    height: 24px;
    border-radius: 2px;
  }
</style>

<div class="label" style="--color: {color}; {style}">
  <p class="typo-text-small-bold">{title}</p>
</div>
