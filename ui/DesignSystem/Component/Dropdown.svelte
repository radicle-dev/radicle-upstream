<script>
  import { Icon, Text } from "../Primitive";
  import Option from "./Dropdown/Option.svelte";

  export let placeholder = null;

  export let options = null;
  export let style = null;

  let expanded = false;

  // bind to this prop from the outside
  export let value = null;
  export let disabled = false;

  const toggleMenu = () => {
    if (disabled) {
      return;
    }

    expanded = !expanded;
  };
  const hideMenu = () => {
    expanded = false;
  };

  const optionSelectedHandler = event => {
    value = event.detail.value;
    toggleMenu();
  };

  const disabledColor = () => {
    return disabled
      ? "var(--color-foreground-level-4)"
      : "var(--color-foreground-level-6)";
  };
</script>

<style>
  .dropdown {
    position: relative;
  }

  .dropdown > * {
    min-width: 187px;
  }

  .button {
    height: 48px;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    display: flex;
    align-items: center;
    user-select: none;
    display: flex;
    justify-content: space-between;
    overflow: hidden; /* hack to make inner option corners rounded */
  }

  .button:hover {
    box-shadow: 0px 0px 0px 1px var(--color-foreground-level-3);
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground);
  }

  .button[hidden] {
    visibility: hidden;
  }

  .menu {
    position: absolute;
    top: 0px;
    left: 0px;
    box-shadow: var(--elevation-medium),
      0px 0px 0px 1px var(--color-foreground-level-3);
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    user-select: none;
    background-color: var(--color-background);
    overflow: hidden; /* hack to make inner option corners rounded */
    z-index: 1;
  }
</style>

<svelte:window on:click={hideMenu} />

<div class="dropdown" {style}>
  <div class="button" on:click|stopPropagation={toggleMenu}>
    {#if value}
      <Option {...options.find(option => option.value === value)} {disabled} />
    {:else}
      <Text style={`margin-left: 12px; color: ${disabledColor()}`}>
        {placeholder}
      </Text>
    {/if}
    <Icon.Expand style={`margin: 0 8px 0 8px; fill: ${disabledColor()}`} />
  </div>

  <div class="menu" hidden={!expanded}>
    {#each options as option}
      <Option {...option} on:selected={optionSelectedHandler} />
    {/each}
  </div>
</div>
