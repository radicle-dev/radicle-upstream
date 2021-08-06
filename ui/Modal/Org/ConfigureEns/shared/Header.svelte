<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  export let title: string;
  export let description: string;
  export let style: string | undefined = undefined;
</script>

<style>
  h1 {
    margin-bottom: 16px;
  }

  p {
    color: var(--color-foreground-level-6);
  }
</style>

<div {style}>
  <h1>{title}</h1>
  <p>{description}</p>
</div>
