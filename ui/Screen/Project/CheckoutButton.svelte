<script>
  import { createEventDispatcher } from "svelte";

  import { Button, Input, Icon } from "../../DesignSystem/Primitive";
  import { Tooltip } from "../../DesignSystem/Component";

  const dispatch = createEventDispatcher();

  // Dropdown element. Set by the view.
  let dropdown = null;
  // Dropdown state.
  let expanded = false;

  const toggleDropdown = ev => {
    expanded = !expanded;
    ev && ev.stopPropagation();
  };

  const clickOutside = ev => {
    // Any click *outside* the dropdown should hide the dropdown.
    if (expanded && dropdown !== ev.target && !dropdown.contains(ev.target)) {
      expanded = false;
    }
  };

  let checkoutDirectoryPath;
</script>

<style>
  .clone-dropdown {
    margin-top: 0.5rem;
    top: 3.25rem;
    right: 0;
    position: absolute;
    border-radius: 4px;
    background: var(--color-background);
    border: 1px solid var(--color-foreground-level-3);
    box-shadow: var(--elevation-medium);
    padding: 1rem;
  }
  p {
    color: var(--color-foreground-level-6);
    user-select: none;
    margin-bottom: 16px;
  }
</style>

<svelte:window on:click={clickOutside} />

<div class="clone-dropdown" hidden={!expanded} bind:this={dropdown}>
  <p>Checkout a working copy to your local disk</p>

  <Input.Directory
    style="margin-bottom: 16px;"
    placeholder="~/path/to/folder"
    buttonVariant="outline"
    bind:path={checkoutDirectoryPath} />

  <Tooltip
    value={!checkoutDirectoryPath ? 'Please select a folder' : null}
    position="bottom">
    <Button
      on:click={() => {
        dispatch('checkout', { checkoutDirectoryPath: checkoutDirectoryPath });
        toggleDropdown();
      }}
      disabled={!checkoutDirectoryPath}
      variant="secondary"
      style="width: 100%; display: block; text-align: center;">
      Checkout
    </Button>
  </Tooltip>
</div>

<Button variant="transparent" icon={Icon.Copy} on:click={toggleDropdown}>
  Checkout
</Button>
