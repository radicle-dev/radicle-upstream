<script>
  import { Text } from "../Primitive";
  export let value = null;
  export let position = "right"; // right | bottom | top | left

  let tooltip = { className: "hidden" };

  const hideTooltip = () => {
    tooltip.className = "hidden";
  };
  const showTooltip = e => {
    const rect = e.target.closest("[data-tooltip]").getBoundingClientRect();
    const offsetY = rect.height < 32 ? (32 - rect.height) / 2 : 0;
    if (position === "right") {
      tooltip = {
        positionY: rect.top - offsetY,
        positionX: rect.right + 8,
        className: "visible",
      };
    } else if (position === "left") {
      tooltip = {
        positionY: rect.top - offsetY,
        positionX: rect.left - rect.width - 24,
        className: "visible",
      };
    } else if (position === "bottom") {
      tooltip = {
        positionY: rect.bottom + 8,
        positionX: rect.left + rect.width / 2,
        className: "visible",
      };
    } else if (position === "top") {
      tooltip = {
        positionY: rect.top - 40,
        positionX: rect.left + rect.width / 2,
        className: "visible",
      };
    }
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

<div
  style="height: 100%;"
  data-tooltip
  on:mouseover={event => showTooltip(event)}
  on:mouseout={hideTooltip}>
  <slot />
  <div
    style={`top: ${tooltip.positionY}px; left: ${tooltip.positionX}px;`}
    class={`tooltip ${tooltip.className} ${position}`}>
    <Text>{value}</Text>
  </div>
  <span class="triangle" />
</div>
