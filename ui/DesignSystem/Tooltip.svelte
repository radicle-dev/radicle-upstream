<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { CSSPosition } from "ui/src/style";
  import { calculatePosition, Visibility } from "ui/src/tooltip";

  export let value = "";

  export let position: CSSPosition = CSSPosition.Right;

  let container: Element | null = null;
  let message: Element | null = null;
  let tooltip = { className: Visibility.Hidden, x: 0, y: 0 };

  const hide = () => {
    tooltip.className = Visibility.Hidden;
  };

  const show = (_event: MouseEvent) => {
    if (!container || !message) {
      // This can never happen: `show` can only be triggered if
      // `container` and `message` have been bound.
      throw new Error("Unreachable: Component DOM nodes not properly bound");
    }

    const containerRect = container.getBoundingClientRect();
    const messageRect = message.getBoundingClientRect();

    tooltip = {
      className: Visibility.Visible,
      ...calculatePosition(position, containerRect, messageRect),
    };
  };
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

  .tooltip.visible {
    visibility: visible;
  }

  .tooltip.hidden {
    visibility: hidden;
  }
</style>

{#if value.length > 0}
  <div
    bind:this={container}
    data-cy="tooltip"
    on:mouseenter={show}
    on:mouseleave={hide}>
    <slot />
    <div
      bind:this={message}
      class={`tooltip ${tooltip.className} ${position}`}
      style={`top: ${tooltip.y}px; left: ${tooltip.x}px;`}>
      <p>{value}</p>
    </div>
  </div>
{:else}
  <slot />
{/if}
