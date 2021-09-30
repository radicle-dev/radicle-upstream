<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript" context="module">
  export type Position = "top" | "right" | "bottom" | "left";
</script>

<script lang="typescript">
  type Offset = { top: number; left: number };

  export let style: string | undefined = undefined;
  export let value: string | undefined = undefined;
  export let position: Position = "right";

  let container: Element | null = null;
  let message: Element | null = null;

  let visible: boolean;
  let offset: Offset = {
    top: 0,
    left: 0,
  };

  const TOOLTIP_MARGIN = 8;

  function hide() {
    visible = false;
  }

  function show() {
    if (!container || !message) {
      // This can never happen: `show` can only be triggered if
      // `container` and `message` have been bound.
      throw new Error("Unreachable: Component DOM nodes not properly bound");
    }

    const containerRect = container.getBoundingClientRect();
    const messageRect = message.getBoundingClientRect();

    visible = true;
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
          top: -(TOOLTIP_MARGIN + message.height),
          left: (container.width - message.width) / 2,
        };

      case "right":
        return {
          top: (container.height - message.height) / 2,
          left: container.width + TOOLTIP_MARGIN,
        };

      case "bottom":
        return {
          top: container.height + TOOLTIP_MARGIN,
          left: (container.width - message.width) / 2,
        };

      case "left":
        return {
          top: (container.height - message.height) / 2,
          left: -(message.width + TOOLTIP_MARGIN),
        };
    }
  }
</script>

<style>
  .container {
    position: relative;
  }

  .tooltip {
    white-space: nowrap;
    user-select: none;
    background-color: var(--color-foreground);
    color: var(--color-background);
    border-radius: 0.5rem;
    padding: 4px 8px;
    position: absolute;
    pointer-events: none;
    z-index: 100;
    visibility: hidden;
  }

  .visible {
    visibility: visible;
  }
</style>

{#if value}
  <div
    {style}
    class="container"
    bind:this={container}
    data-cy="tooltip"
    on:mouseenter={show}
    on:mouseleave={hide}>
    <slot />
    <div
      bind:this={message}
      class="tooltip"
      class:visible
      style={`top: ${offset.top}px; left: ${offset.left}px`}>
      {value}
    </div>
  </div>
{:else}
  <slot />
{/if}
