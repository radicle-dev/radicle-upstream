<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { debounce } from "lodash";

  export let disabled: boolean = false;
  export let modalStyle: string | undefined = undefined;
  export let style: string | undefined = undefined;

  export let onShow: () => void = () => {};

  let visible: boolean = false;

  const setVisible = debounce((value: boolean) => {
    if (!disabled) {
      visible = value;
      if (visible) {
        onShow();
      }
    }
  }, 50);
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

<div
  {style}
  on:mouseenter={() => setVisible(true)}
  on:mouseleave={() => setVisible(false)}>
  <slot name="trigger" />

  {#if visible}
    <!-- The `stopPropagation` is necessary in case the `<Hovercard>` is ontop of
       another element which itself is clickable. Otherwise any click within
       the `<Hovercard>` would also trigger the click handler from the element
       underneath it. -->
    <div class="container" on:click|stopPropagation>
      <div class="modal" style={modalStyle}>
        <slot name="card" />
      </div>
    </div>
  {/if}
</div>
