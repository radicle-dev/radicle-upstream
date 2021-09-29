<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  export let value: string | undefined = undefined;
  export let style: string | undefined = undefined;
</script>

<style>
  input[type="color"] {
    -webkit-appearance: none;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    width: 2.5rem;
    height: 2.5rem;
  }
  input[type="color"]::-webkit-color-swatch-wrapper {
    padding: 0;
  }
  input[type="color"]::-webkit-color-swatch {
    border: none;
    border-radius: 0.25rem;
    margin: 0.25rem;
  }
</style>

<input {style} type="color" bind:value />
