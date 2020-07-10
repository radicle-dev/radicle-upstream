<script>
  import { Button, Input, Icon, Text } from "../../DesignSystem/Primitive";

  import * as notification from "../../src/notification.ts";

  export let projectId = null;
  export let projectName = null;

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

  let checkoutDirectoryPath;

  const handleCheckout = () => {
    projectId;
    notification.info(`${projectName} checked out to ${checkoutDirectoryPath}`);
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
  <Text
    style="color: var(--color-foreground-level-6); user-select: none;
    margin-bottom: 16px;">
    Checkout a working copy to your local disk
  </Text>

  <Input.Directory
    style="margin-bottom: 16px;"
    placeholder="~/path/to/folder"
    buttonVariant="outline"
    bind:path={checkoutDirectoryPath} />

  <Button
    on:click={handleCheckout}
    disabled={!checkoutDirectoryPath}
    title={!checkoutDirectoryPath ? 'Please select a folder.' : ''}
    variant="secondary"
    style="width: 100%; display: block; text-align: center;">
    Checkout
  </Button>
</div>

<Button variant="transparent" icon={Icon.Copy} on:click={toggleDropdown}>
  Checkout
</Button>
