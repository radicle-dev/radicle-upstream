<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Tooltip from "../Tooltip.svelte";

  export let style: string | undefined = undefined;
  export let color: string;
</script>

<style>
  .wrapper {
    display: inline-flex;
  }
  .color {
    width: 4rem;
    height: 4rem;
    border-radius: 0.5rem;
  }
  .border {
    border: 1px solid var(--color-foreground-level-3);
  }
</style>

<div class="wrapper" {style}>
  <Tooltip value={`var(${color})`}>
    <div
      class:border={color === "--color-background"}
      class="color"
      style={`background-color: var(${color});`} />
  </Tooltip>
</div>
