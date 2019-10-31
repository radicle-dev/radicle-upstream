<style>
  div {
    background-color: var(--color-darkgray);
    color: var(--color-white);
    text-align: center;
    border-radius: 2px;
    padding: 8px 12px;
    transition: opacity 100ms ease;

    /* Position the tooltip */
    position: absolute;
    z-index: 1;
    top: -6px;
    left: 40px;
  }

  div:before {
    content: "";
    display: block;
    width: 0;
    height: 0;
    position: absolute;

    border-top: 6px solid transparent;
    border-bottom: 6px solid transparent;
    border-right: 6px solid var(--color-darkgray);
    left: -6px;

    top: 12px;
  }

  div:hover {
    opacity: 0;
  }
</style>

<div class="show-on-hover">
  <slot />
</div>
