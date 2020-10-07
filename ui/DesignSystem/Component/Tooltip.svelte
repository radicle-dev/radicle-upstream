<script lang="ts">
  import { CSSPosition } from "../../src/style";

  export let value = "";

  export let position: CSSPosition = CSSPosition.Right;

  enum Visibility {
    Hidden = "hidden",
    Visible = "visible",
  }

  let tooltip = { className: Visibility.Hidden, positionX: 0, positionY: 0 };

  const hideTooltip = () => {
    tooltip.className = Visibility.Hidden;
  };

  // TODO(sos): there may exist a better way to type this event
  const showTooltip = (e: {
    target: { closest: (div: string) => HTMLDivElement };
  }) => {
    const rect = e.target.closest("[data-tooltip]").getBoundingClientRect();
    const offsetY = rect.height < 32 ? (32 - rect.height) / 2 : 0;
    if (position === CSSPosition.Right) {
      tooltip = {
        positionY: rect.top - offsetY,
        positionX: rect.right + 8,
        className: Visibility.Visible,
      };
    } else if (position === CSSPosition.Left) {
      tooltip = {
        positionY: rect.top - offsetY,
        positionX: rect.left - rect.width - 24,
        className: Visibility.Visible,
      };
    } else if (position === CSSPosition.Bottom) {
      tooltip = {
        positionY: rect.bottom + 8,
        positionX: rect.left + rect.width / 2,
        className: Visibility.Visible,
      };
    } else if (position === CSSPosition.Top) {
      tooltip = {
        positionY: rect.top - 40,
        positionX: rect.left + rect.width / 2,
        className: Visibility.Visible,
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

{#if value.length > 0}
  <div
    style="height: 100%;"
    data-tooltip
    data-cy="tooltip"
    on:mouseenter={event => showTooltip(event)}
    on:mouseleave={hideTooltip}>
    <slot />
    <div
      style={`top: ${tooltip.positionY}px; left: ${tooltip.positionX}px;`}
      class={`tooltip ${tooltip.className} ${position}`}>
      <p>{value}</p>
    </div>
    <span class="triangle" />
  </div>
{:else}
  <slot />
{/if}
