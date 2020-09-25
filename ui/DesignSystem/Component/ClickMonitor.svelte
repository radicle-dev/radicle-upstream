<script lang="ts">
  import { closeCurrentDropdown, currentlyOpen } from "../../src/dropdown";

  const handleClick = (ev: MouseEvent) => {
    const component = $currentlyOpen;
    const inside = component && component.contains(ev.target);
    if (!inside) closeCurrentDropdown();
  };
</script>

<svelte:window on:click={handleClick} />
