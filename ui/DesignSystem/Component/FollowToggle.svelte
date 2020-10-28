<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { Icon } from "../Primitive";
  import Hoverable from "./Hoverable.svelte";

  export let disabled: boolean = false;
  export let expanded: boolean = false;
  export let following: boolean = false;
  export let warning: boolean = false;
  export let style = "";

  let active: boolean = false;

  const dispatch = createEventDispatcher();

  const down = () => {
    if (disabled) return;

    active = true;
  };

  const up = () => {
    if (disabled) return;

    active = false;
    following = !following;
    dispatch(following ? "follow" : "unfollow");
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
  }
  .toggle.active {
    border: 1px solid var(--color-secondary-level-1);
    background-color: var(--color-secondary-level-1);
  }
  .toggle.following {
    border: 1px solid var(--color-foreground-level-3);
    background-color: var(--color-foreground-level-3);
    color: var(--color-foreground-level-6);
  }
  .toggle.following.hover {
    border: 1px solid var(--color-foreground-level-2);
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
  }
  .toggle.following.hover.warning {
    color: var(--color-background);
    border: none;
  }
  .toggle.following.active {
    border: 1px solid var(--color-foreground-level-2);
    background-color: var(--color-foreground-level-2);
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
  .toggle .left.following :global(svg) {
    fill: var(--color-foreground-level-6);
  }
  .left.following {
    background-color: var(--color-foreground-level-3);
  }
  .left.following.hover {
    background-color: var(--color-foreground-level-2);
  }
  .left.following.hover :global(svg) {
    fill: var(--color-foreground-level-5);
  }
  .left.following.active {
    background-color: var(--color-foreground-level-2);
  }
  .left.following.active :global(svg) {
    fill: var(--color-foreground-level-4);
  }
  .left.following.warning.hover {
    background-color: var(--color-negative);
  }
  .left.following.warning.hover :global(svg) {
    fill: var(--color-background);
  }
  .disabled {
    cursor: not-allowed;
  }
</style>

<Hoverable let:hovering={hover}>
  <div
    data-cy="follow-toggle"
    class:active
    class:hover={hover && !disabled}
    class:following
    class:disabled
    class:warning
    class="toggle"
    {style}
    on:mousedown={() => {
      !disabled && down();
    }}
    on:mouseup={() => {
      !disabled && up();
    }}>
    <div
      class="left"
      class:active
      class:hover={hover && !disabled}
      class:following
      class:warning>
      {#if !following}
        <Icon.Network style="margin: 0 8px 0 12px" />
        <p class="typo-text-bold" style="margin-right: 12px">Follow</p>
      {:else if hover && !disabled}
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
