<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  // Wrapper for expanding/dismissing overlays

  import { createEventDispatcher } from "svelte";

  import { close, current, open } from "ui/src/overlay";

  export let expanded: boolean;
  export let style: string = "";

  let container: HTMLDivElement;

  const dispatch = createEventDispatcher();

  const handleClick = (ev: MouseEvent) => {
    const component = $current;
    const inside = component && component.contains(ev.target as HTMLDivElement);
    if (!inside) {
      close();
    }
  };

  $: if (expanded) {
    open(container);
  }
  $: if ($current !== container) {
    dispatch("hide");
  }
</script>

<svelte:window on:click={handleClick} />

<div bind:this={container} {style}>
  <slot />
</div>
