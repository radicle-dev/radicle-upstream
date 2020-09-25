<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import {
    openDropdown,
    closeCurrentDropdown,
    currentlyOpen,
  } from "../../src/overlay";

  export let expand: boolean;
  export let style = "";

  let container;

  const handleClick = (ev: MouseEvent) => {
    const component = $currentlyOpen;
    const inside = component && component.contains(ev.target);
    if (!inside) closeCurrentDropdown();
  };

  const dispatch = createEventDispatcher();

  const dismiss = () => dispatch("dismiss");
  const open = () => openDropdown(container);

  $: if (expand) open();
  $: if ($currentlyOpen !== container) dismiss();
</script>

<!-- Simple wrapper for expanded/dismissing overlays -->
<svelte:window on:click={handleClick} />

<div bind:this={container} {style}>
  <slot />
</div>
