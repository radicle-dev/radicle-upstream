<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let title: string;
  export let selected: boolean = false;
  export let value: string;
  export let style: string | undefined = undefined;
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
  <p
    class="typo-overflow-ellipsis"
    style={`margin: 0 42px  0 12px; color: ${disabledColor}`}>
    {title}
  </p>
</div>
