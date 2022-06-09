<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  type Offset = { top: number; left: number };
  import type { Position } from "./Tooltip";

  import { debounce } from "lodash";

  export let style: string | undefined = undefined;
  export let value: string | undefined = undefined;
  export let position: Position = "right";
  export let showDelay = 50; // ms

  let container: Element | null = null;
  let message: Element | null = null;
  let offset: Offset = { top: 0, left: 0 };
  let visible: boolean = false;

  const setVisible = debounce((value: boolean) => {
    visible = value;
  }, showDelay);

  function calculateOffset(
    position: Position,
    container: DOMRect,
    message: DOMRect
  ): Offset {
    switch (position) {
      case "top":
        return {
          top: container.top - 40,
          left: container.left + container.width / 2,
        };

      case "right":
        return {
          top: container.top + container.height / 2 - 16,
          left: container.right + 8,
        };

      case "bottom":
        return {
          top: container.bottom + 8,
          left: container.left + container.width / 2,
        };

      case "left":
        return {
          top: container.top + container.height / 2 - 16,
          left: container.left - message.width - 8,
        };
    }
  }

  $: if (container && message) {
    offset = calculateOffset(
      position,
      container.getBoundingClientRect(),
      message.getBoundingClientRect()
    );
  }
</script>

<style>
  .tooltip {
    white-space: nowrap;
    user-select: none;
    background-color: var(--color-foreground);
    color: var(--color-background);
    text-align: center;
    border-radius: 0.5rem;
    padding: 4px 8px;
    position: fixed;
    pointer-events: none;
    z-index: 100;
  }

  .tooltip.bottom,
  .tooltip.top {
    transform: translateX(-50%);
  }
</style>

{#if value}
  <div
    {style}
    bind:this={container}
    on:mouseenter={() => setVisible(true)}
    on:mouseleave={() => setVisible(false)}>
    <slot />

    {#if visible}
      <div
        bind:this={message}
        class={`typo-text tooltip ${position}`}
        style:top={`${offset.top}px`}
        style:left={`${offset.left}px`}>
        {value}
      </div>
    {/if}
  </div>
{:else}
  <slot />
{/if}
