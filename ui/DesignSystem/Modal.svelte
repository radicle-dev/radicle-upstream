<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  export let dataCy: string = "";
  export let style: string = "";
</script>

<style>
  .container {
    width: 37.5rem;
    background: var(--color-background);
    border-radius: 1rem;
    padding: 2rem;
    box-shadow: var(--color-shadows);
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .container:focus {
    outline: none;
  }
  .content {
    text-align: center;
    width: 100%;
  }
</style>

<div class="container" data-cy={dataCy}>
  <div class="content" {style}>
    <slot />
  </div>
</div>
