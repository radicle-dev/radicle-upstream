<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { fly } from "svelte/transition";
  import { backOut } from "svelte/easing";

  import {
    primaryColor,
    primaryColorOptions,
    primaryColorHex,
  } from "ui/src/appearance";

  import SegmentedControl from "design-system/SegmentedControl.svelte";
</script>

<style>
  input[type="color"] {
    -webkit-appearance: none;
    margin: 0.25rem;
    width: 1.875rem;
    height: 1.875rem;
  }
  input[type="color"]::-webkit-color-swatch-wrapper {
    padding: 0;
    cursor: pointer;
  }
  input[type="color"]::-webkit-color-swatch {
    border: none;
    border-radius: 0.25rem;
  }
</style>

<SegmentedControl
  active={$primaryColor}
  options={primaryColorOptions}
  on:select={ev => primaryColor.set(ev.detail)}>
  {#if $primaryColor === "custom"}
    <input
      in:fly|local={{ x: 30, duration: 100, easing: backOut }}
      out:fly|local={{ x: 30, duration: 100 }}
      type="color"
      bind:value={$primaryColorHex} />
  {/if}
</SegmentedControl>
