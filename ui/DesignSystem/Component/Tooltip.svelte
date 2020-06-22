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
    border-radius: 6px;
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
    width: 0;
    height: 0;
    position: absolute;
    border-top: 6px solid transparent;
    border-bottom: 6px solid transparent;
    border-right: 6px solid var(--color-foreground);
    left: -6px;
    border-top-left-radius: 30%;
    top: 10px;
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
</div>
