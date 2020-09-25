<script lang="ts">
  // Wrapper for expanding/dismissing overlays

  import { createEventDispatcher } from "svelte";

  import { openOverlay, closeOverlay, currentlyOpen } from "../../src/overlay";

  export let expanded: boolean;
  export let style = "";

  let container;

  const dispatch = createEventDispatcher();

  const hide = () => dispatch("hide");
  const open = () => openOverlay(container);

  const handleClick = (ev: MouseEvent) => {
    const component = $currentlyOpen;
    const inside = component && component.contains(ev.target);
    if (!inside) closeOverlay();
  };

  $: if (expanded) open();
  $: if ($currentlyOpen !== container) hide();
</script>

<svelte:window on:click={handleClick} />

<div bind:this={container} {style}>
  <slot />
</div>
