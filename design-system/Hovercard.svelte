<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { fade } from "svelte/transition";
  import Hoverable from "./Hoverable.svelte";

  export let onHover: () => void = () => {};
  export let style: string | undefined = undefined;
  export let modalStyle: string | undefined = undefined;
  export let disabled: boolean = false;

  let hover: boolean = false;

  $: {
    if (!disabled && hover) {
      onHover();
    }
  }
</script>

<style>
  .container {
    position: absolute;
  }

  .modal {
    position: absolute;
    border-radius: 1rem;
    background: var(--color-background);
    box-shadow: var(--elevation-high);
    top: -1rem;
    left: 2rem;
    z-index: 1;
  }
</style>

<Hoverable bind:hovering={hover} {style}>
  <slot name="trigger" />

  <!-- The `stopPropagation` is necessary in case the `<Hovercard>` is ontop of
       another element which itself is clickable. Otherwise any click within
       the hover card would also trigger the click handler from the element
       underneath it. -->
  <div class="container" on:click|stopPropagation>
    {#if !disabled && hover}
      <div
        class="modal"
        style={modalStyle}
        out:fade|local={{ duration: 100, delay: 250 }}>
        <slot name="card" />
      </div>
    {/if}
  </div>
</Hoverable>
