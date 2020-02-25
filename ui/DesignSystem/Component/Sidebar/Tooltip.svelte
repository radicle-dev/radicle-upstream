<style>
  .tooltip {
    user-select: none;
    background-color: var(--color-white);
    border: 1px solid var(--color-lightgray);
    color: var(--color-darkgray);
    text-align: center;
    border-radius: 2px;
    padding: 4px 10px 6px 8px;
    box-shadow: 0px 4px 8px rgba(51, 51, 51, 0.08);

    /* Position the tooltip */
    position: absolute;
    z-index: 1;
    top: 2px;
    left: 48px;
  }

  .tooltip:after,
  .tooltip:before {
    right: 100%;
    top: 50%;
    border: solid transparent;
    content: " ";
    height: 0;
    width: 0;
    position: absolute;
    pointer-events: none;
  }

  .tooltip:after {
    border-right-color: var(--color-white);
    border-width: 6px;
    margin-top: -6px;
  }

  .tooltip:before {
    border-right-color: var(--color-lightgray);
    border-width: 7px;
    margin-top: -7px;
  }

  .tooltip:hover {
    opacity: 0;
  }
</style>

<div class="tooltip show-on-hover">
  <slot />
</div>
