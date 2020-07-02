<script>
  import { Button, Code, Icon, Text } from "../../DesignSystem/Primitive";
  import { Copyable } from "../../DesignSystem/Component";

  export let projectId = null;

  // Dropdown element. Set by the view.
  let dropdown = null;
  // Dropdown state.
  let expanded = false;
  // Icon to show next to clone command.
  let copyIcon = Icon.Copy;

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

  const afterCopy = () => {
    copyIcon = Icon.Check;
    setTimeout(() => {
      copyIcon = Icon.Copy;
    }, 1000);
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
    /* TODO(julien): Replace with `elevation-medium`. */
    box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.12), 0px 0px 1px rgba(0, 0, 0, 0.12);
    padding: 1rem;
  }
  .clone-dropdown .code-block {
    border-radius: 4px;
    color: var(--color-foreground);
    background: var(--color-foreground-level-2);
    padding: 0.5rem;
    margin-top: 0.5rem;
    display: flex;
    align-items: center;
  }
</style>

<svelte:window on:click={clickOutside} />
<div class="clone-dropdown" hidden={!expanded} bind:this={dropdown}>
  <Text style="color: var(--color-foreground-level-6); user-select: none">
    Clone this repository using the following URL.
  </Text>
  <Copyable {afterCopy}>
    <div class="code-block">
      <Code style="padding-right: 0.5rem">{projectId}</Code>
      <svelte:component this={copyIcon} />
    </div>
  </Copyable>
</div>
<Button variant="transparent" icon={Icon.Copy} on:click={toggleDropdown}>
  Clone
</Button>
