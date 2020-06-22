<script>
  import { Text } from "../Primitive";
  export let value = null;

  let tooltip = { className: "hidden" };

  const hideTooltip = () => {
    tooltip.className = "hidden";
  };
  const showTooltip = e => {
    const rect = e.target.closest("[data-tooltip]").getBoundingClientRect();
    tooltip = { positionY: rect.top, className: "visible" };
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

    left: 64px;
    pointer-events: none;
    z-index: 100;
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
    bottom: calc(50% - 4px);
    left: -4px;
    clip-path: polygon(0% 0%, 100% 100%, 0% 100%);
    transform: rotate(45deg);
    border-radius: 0 0 0 0.1875rem;
  }
</style>

<div
  style="height: 100%;"
  data-tooltip
  on:mouseover={event => showTooltip(event)}
  on:mouseout={hideTooltip}>
  <slot />
  <div
    style={`top: ${tooltip.positionY}px`}
    class={`tooltip ${tooltip.className}`}>
    <Text>{value}</Text>
  </div>
  <span class="triangle" />
</div>
