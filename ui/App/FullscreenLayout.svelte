<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as router from "ui/src/router";

  import { Button, Icon } from "ui/DesignSystem";

  export let style = "";
  export let contentStyle = "";
  export let escapable: boolean = true;
  export let onClose = router.pop;

  const onKeydown = (event: KeyboardEvent) => {
    if (
      escapable &&
      event.target === document.body &&
      event.code === "Escape"
    ) {
      onClose();
    }
  };
</script>

<style>
  .close {
    cursor: pointer;
    position: absolute;
    right: 32px;
    top: 22px;
  }

  .fullscreen {
    align-items: center;
    display: flex;
    height: 100vh;
    justify-content: center;
    overflow: auto;
    width: 100vw;
  }

  .content {
    overflow: visible;
    height: 100%;
    width: 100%;
  }
</style>

<svelte:window on:keydown={onKeydown} />

{#if escapable}
  <div class="close transition">
    <Button style="padding:0.5rem;" on:click={onClose} variant="transparent">
      <Icon.Cross />
    </Button>
  </div>
{/if}

<div class="fullscreen" {style}>
  <div class="content" style={contentStyle}>
    <slot />
  </div>
</div>
