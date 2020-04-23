<script>
  import { Icon, Text } from "../Primitive";
  import Option from "./Dropdown/Option.svelte";
  import { orgMocks } from "../../lib/orgMocks.js";

  export let placeholder = "Select issue type";

  const org = orgMocks.data.orgs[0];
  const identity = {
    id: "123abcd.git",
    shareableEntityIdentifier: "cloudhead@123abcd.git",
    metadata: {
      handle: "cloudhead",
      displayName: "Alexis Sellier",
      avatarUrl: "https://avatars1.githubusercontent.com/u/40774"
    },
    registered: null,
    avatarFallback: { background: { r: 122, g: 112, b: 90 }, emoji: "💡" }
  };

  export let options = [
    { text: "Option 1", value: "1", variant: "text" },
    { text: "Option 2", value: "2", variant: "org", org: org },
    { text: "Option 3", value: "3", variant: "identity", identity: identity }
  ];

  let expanded = false;

  // bind to this prop from the outside
  export let value = null;

  const toggleMenu = () => {
    expanded = !expanded;
  };
  const hideMenu = () => {
    expanded = false;
  };

  const optionSelectedHandler = event => {
    value = event.detail.value;
    toggleMenu();
  };

  $: console.log(value);
</script>

<style>
  .dropdown {
    position: relative;
  }

  .dropdown > * {
    min-width: 187px;
  }

  .button {
    height: 40px;
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
    height: 40px;
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

<div class="dropdown">
  <div class="button" on:click|stopPropagation={toggleMenu}>
    {#if value}
      <Option {...options.find(option => option.value === value)} />
    {:else}
      <Text style="margin-left: 12px; color: var(--color-foreground-level-6);">
        {placeholder}
      </Text>
    {/if}
    <Icon.Expand style="margin: 0 8px 0 8px;" />
  </div>

  <div class="menu" hidden={!expanded}>
    {#each options as option}
      <Option {...option} on:selected={optionSelectedHandler} />
    {/each}
  </div>
</div>
