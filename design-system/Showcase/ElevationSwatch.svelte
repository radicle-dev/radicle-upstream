<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Tooltip from "../Tooltip.svelte";

  export let style: string | undefined = undefined;
  export let elevation: string;
</script>

<style>
  .wrapper {
    display: inline-flex;
  }
  .elevation {
    width: 8rem;
    height: 8rem;
    border-radius: 1rem;
  }
</style>

<div class="wrapper" {style}>
  <Tooltip value={`var(${elevation})`} position="top">
    <div class="elevation" style={`box-shadow: var(${elevation});`} />
  </Tooltip>
</div>
