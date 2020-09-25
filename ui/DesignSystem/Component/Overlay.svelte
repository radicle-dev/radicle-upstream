<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { openDropdown, currentlyOpen } from "../../src/dropdown";

  let container;
  export let expand: boolean;

  const dispatch = createEventDispatcher();

  const dismiss = () => dispatch("dismiss");

  const open = () => openDropdown(container);

  $: if ($currentlyOpen !== container) dismiss();
  $: if (expand) open();
</script>

<div bind:this={container}>
  <slot />
</div>
