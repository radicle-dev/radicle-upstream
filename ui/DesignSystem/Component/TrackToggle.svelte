<script>
  import { createEventDispatcher } from "svelte";

  import { Icon } from "../Primitive";
  import Hoverable from "./Hoverable.svelte";

  export let style = null;
  export let tracking = false;
  export let expanded = false;

  export let warning = false;

  let active = false;

  const TrackingEvent = {
    Track: "track",
    Untrack: "untrack",
  };

  const down = () => {
    active = true;
  };

  const up = () => {
    active = false;
    tracking = !tracking;
    dispatch(tracking ? TrackingEvent.Track : TrackingEvent.Untrack);
  };

  const dispatch = createEventDispatcher();
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
  .toggle.tracking.hover.warning {
    color: var(--color-background);
    border: none;
    box-shadow: none;
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

  .left.tracking.warning.hover {
    background-color: var(--color-negative);
  }
  .left.tracking.warning.hover :global(svg) {
    fill: var(--color-background);
  }
</style>

<Hoverable let:hovering={hover}>
  <div
    class:hover
    class:active
    class:tracking
    class:warning
    class="toggle"
    {style}
    on:mousedown={down}
    on:mouseup={up}>
    <div class="left" class:hover class:active class:tracking class:warning>
      {#if !tracking}
        <Icon.Network style="margin: 0 8px 0 12px" />
        <p class="typo-text-bold" style="margin-right: 12px">Follow</p>
      {:else if hover}
        <Icon.Network style="margin: 0 8px 0 12px" />
        <p class="typo-text-bold" style="margin-right: 12px">Unfollow</p>
      {:else if expanded}
        <Icon.Network style="margin: 0 12px" />
        <p class="typo-text-bold" style="margin-right: 12px">Following</p>
      {:else}
        <Icon.Network style="margin: 0 12px" />
      {/if}
    </div>
  </div>
</Hoverable>
