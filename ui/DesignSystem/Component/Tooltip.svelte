<script lang="typescript">
  import { CSSPosition } from "../../src/style";
  import { calculatePosition, Visibility } from "../../src/tooltip";

  export let value = "";

  export let position: CSSPosition = CSSPosition.Right;

  let container: Element | null = null;
  let message: Element | null = null;
  let tooltip = { className: Visibility.Hidden, x: 0, y: 0 };

  const hide = () => {
    tooltip.className = Visibility.Hidden;
  };

  const show = (_event: MouseEvent) => {
    if (!container) {
      console.error("container element not present");
      return;
    }
    if (!message) {
      console.error("message element not present");
      return;
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
    border-radius: 4px;
    padding: 4px 8px;
    position: fixed;

    pointer-events: none;
    z-index: 100;
  }

  .tooltip.bottom,
  .tooltip.top {
    transform: translateX(-50%);
  }
  .tooltip.visible {
    visibility: visible;
  }
  .tooltip.hidden {
    visibility: hidden;
  }
  .tooltip:before {
    content: "";
    display: block;
    height: 8px;
    width: 8px;
    background-color: inherit;
    border: inherit;
    position: absolute;
    clip-path: polygon(0% 0%, 100% 100%, 0% 100%);
    border-radius: 0 0 0 0.1875rem;
  }
  .tooltip.right:before {
    bottom: calc(50% - 4px);
    left: -4px;
    transform: rotate(45deg);
  }
  .tooltip.left:before {
    bottom: calc(50% - 4px);
    right: -4px;
    transform: rotate(-135deg);
  }
  .tooltip.bottom:before {
    left: calc(50% - 4px);
    top: -4px;
    transform: rotate(135deg);
  }
  .tooltip.top:before {
    left: calc(50% - 4px);
    bottom: -4px;
    transform: rotate(-45deg);
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
    <span class="triangle" />
  </div>
{:else}
  <slot />
{/if}
