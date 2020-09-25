<script lang="ts">
  import { openDropdown, currentlyOpen } from "../../../src/dropdown";

  let container;
  export let shouldOpen = false;

  const open = () => {
    shouldOpen = true;
    openDropdown(container);
  };

  $: if ($currentlyOpen !== container) shouldOpen = false;
</script>

<div bind:this={container}>
  <slot />
</div>
