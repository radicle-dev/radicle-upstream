<script>
  import { Icon } from "../Primitive";

  export let style = null;
  export let tracking = false;

  let hover = false;
  let active = false;

  const enter = () => {
    hover = true;
  };

  const leave = () => {
    hover = false;
  };

  const down = () => {
    active = true;
  };

  const up = () => {
    active = false;
    tracking = !tracking;
  };
</script>

<style>
  .toggle {
    display: flex;
    height: 40px;
    align-items: center;
    border-radius: 4px;
    border: 1px solid var(--color-secondary);
    background-color: var(--color-secondary);
    color: var(--color-background);
    cursor: pointer;
    user-select: none;
  }
  .toggle.hover {
    border: 1px solid var(--color-secondary-level-2);
    background-color: var(--color-secondary-level-2);
    box-shadow: 0 0 0 1px var(--color-secondary-level-2);
  }
  .toggle.active {
    border: 1px solid var(--color-secondary-level-1);
    background-color: var(--color-secondary-level-1);
    box-shadow: 0 0 0 1px var(--color-secondary-level-1);
  }
  .toggle.tracking {
    border: 1px solid var(--color-foreground-level-3);
    background-color: var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
  }
  .toggle.tracking.hover {
    border: 1px solid var(--color-foreground-level-2);
    background-color: var(--color-foreground-level-2);
    box-shadow: 0 0 0 1px var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
  }
  .toggle.tracking.active {
    border: 1px solid var(--color-foreground-level-2);
    background-color: var(--color-foreground-level-2);
    box-shadow: 0 0 0 1px var(--color-foreground-level-2);
    color: var(--color-foreground-level-4);
  }
  .left {
    background-color: var(--color-secondary);
    display: flex;
    height: 40px;
    align-items: center;
    border-radius: 3px;
  }
  .toggle .left :global(svg) {
    fill: var(--color-background);
  }
  .left.hover {
    background-color: var(--color-secondary-level-2);
  }
  .left.active {
    background-color: var(--color-secondary-level-1);
  }
  .toggle .left.tracking :global(svg) {
    fill: var(--color-foreground-level-6);
  }
  .left.tracking {
    background-color: var(--color-foreground-level-3);
  }
  .left.tracking.hover {
    background-color: var(--color-foreground-level-2);
  }
  .left.tracking.hover :global(svg) {
    fill: var(--color-foreground-level-5);
  }
  .left.tracking.active {
    background-color: var(--color-foreground-level-2);
  }
  .left.tracking.active :global(svg) {
    fill: var(--color-foreground-level-4);
  }
</style>

<div
  class:hover
  class:active
  class:tracking
  class="toggle"
  {style}
  on:mouseenter={enter}
  on:mousedown={down}
  on:mouseup={up}
  on:mouseleave={leave}>
  <div class="left" class:hover class:active class:tracking>
    {#if !tracking}
      <Icon.Peer style="margin: 0 8px 0 12px" />
      <p class="bold" style="margin-right: 12px">Track</p>
    {:else if hover}
      <Icon.Peer style="margin: 0 8px 0 12px" />
      <p class="bold" style="margin-right: 12px">Untrack</p>
    {:else}
      <Icon.Peer style="margin: 0 12px" />
    {/if}
  </div>
</div>
