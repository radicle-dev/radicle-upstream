<script>
  import { createEventDispatcher } from "svelte";

  export let exclude = [];

  let child;

  const dispatch = createEventDispatcher();

  function isExcluded(target) {
    let parent = target;

    while (parent) {
      if (exclude.indexOf(parent) >= 0 || parent === child) {
        return true;
      }

      parent = parent.parentNode;
    }

    return false;
  }

  function onClickOutside(event) {
    if (!isExcluded(event.target)) {
      dispatch("clickoutside");
    }
  }
</script>

<!-- Adapted from https://github.com/joeattardi/svelte-click-outside -->
<svelte:window on:click={onClickOutside} />
<div bind:this={child}>
  <slot />
</div>
