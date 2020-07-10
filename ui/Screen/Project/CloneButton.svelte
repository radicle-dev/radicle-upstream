<script>
  import { Button, Code, Icon, Text } from "../../DesignSystem/Primitive";
  import { Copyable } from "../../DesignSystem/Component";

  export let projectId = null;

  // Dropdown element. Set by the view.
  let dropdown = null;
  // Dropdown state.
  let expanded = false;

  const toggleDropdown = ev => {
    expanded = !expanded;
    ev.stopPropagation();
  };

  const clickOutside = ev => {
    // Any click *outside* the dropdown should hide the dropdown.
    if (expanded && dropdown !== ev.target && !dropdown.contains(ev.target)) {
      expanded = false;
    }
  };
</script>

<style>
  .clone-dropdown {
    margin-top: 0.5rem;
    top: 3.25rem;
    right: 0;
    position: absolute;
    border-radius: 4px;
    background: var(--color-background);
    border: 2px solid var(--color-foreground-level-3);
    box-shadow: var(--elevation-medium);
    padding: 1rem;
  }
</style>

<svelte:window on:click={clickOutside} />
<div class="clone-dropdown" hidden={!expanded} bind:this={dropdown}>
  <Text style="color: var(--color-foreground-level-6); user-select: none">
    Clone this repository using the following URL.
  </Text>
  <Copyable
    style="border-radius: 4px; color: var(--color-foreground); background:
    var(--color-foreground-level-2); padding: 0.5rem; margin-top: 0.5rem;
    display: flex; align-items: center;"
    iconSize="normal">
    <Code style="padding-right: 0.5rem">{projectId}</Code>
  </Copyable>
</div>
<Button variant="transparent" icon={Icon.Copy} on:click={toggleDropdown}>
  Clone
</Button>
