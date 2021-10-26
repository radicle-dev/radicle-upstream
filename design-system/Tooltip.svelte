<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Position } from "./Tooltip";

  export let style: string | undefined = undefined;
  export let value: string | undefined = undefined;
  export let position: Position = "right";

  type Offset = { top: number; left: number };

  let container: Element | null = null;
  let message: Element | null = null;

  let visibility: "hidden" | "visible" = "hidden";
  let offset: Offset = {
    top: 0,
    left: 0,
  };

  function hide() {
    visibility = "hidden";
  }

  function show() {
    if (!container || !message) {
      // This can never happen: `show` can only be triggered if
      // `container` and `message` have been bound.
      throw new Error("Unreachable: Component DOM nodes not properly bound");
    }

    const containerRect = container.getBoundingClientRect();
    const messageRect = message.getBoundingClientRect();

    visibility = "visible";
    offset = calculateOffset(position, containerRect, messageRect);
  }

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
    data-cy="tooltip"
    on:mouseenter={show}
    on:mouseleave={hide}>
    <slot />
    <div
      bind:this={message}
      class={`tooltip ${position}`}
      style={`top: ${offset.top}px; left: ${offset.left}px; visibility: ${visibility}`}>
      <p>{value}</p>
    </div>
  </div>
{:else}
  <slot />
{/if}
