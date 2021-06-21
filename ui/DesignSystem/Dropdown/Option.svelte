<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import type { Avatar as AvatarT } from "ui/src/proxy/identity";
  import Avatar from "../Avatar.svelte";

  const dispatch = createEventDispatcher();

  export let title: string;

  // Only allow parents to modify avatar props that make sense in this context --
  // e.g. the `size` should only ever be `small` until this component can accommodate
  // dynamic sizing
  export let avatarProps:
    | {
        avatarFallback: AvatarT;
        title?: string;
        variant?: "circle" | "square";
      }
    | undefined = undefined;
  export let selected: boolean = false;

  export let value: string;
  export let style = "";
  export let disabled: boolean = false;

  const disabledColor = disabled
    ? "var(--color-foreground-level-4)"
    : "var(--color-foreground-level-6)";

  const clickHandler = () => {
    dispatch("selected", { value: value });
  };
</script>

<style>
  .option {
    display: flex;
    height: 38px;
    align-items: center;
    white-space: nowrap;
    overflow: hidden;
  }

  .option.selected,
  .option.selected:hover {
    background-color: var(--color-foreground-level-2);
  }

  .option:hover {
    background-color: var(--color-foreground-level-1);
  }
</style>

<div class="option" on:click={clickHandler} class:selected {style}>
  {#if avatarProps}
    <Avatar
      size="small"
      style="overflow:hidden; text-overflow: ellipsis; margin: 0 42px 0 8px;
      --title-color: var(--color-foreground-level-6);"
      {...avatarProps} />
  {:else}
    <p
      class="typo-overflow-ellipsis"
      style={`margin: 0 42px  0 12px; color: ${disabledColor}`}>
      {title}
    </p>
  {/if}
</div>
