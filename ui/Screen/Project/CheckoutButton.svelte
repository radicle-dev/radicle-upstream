<script>
  import { createEventDispatcher } from "svelte";

  import { Button, Input, Icon } from "../../DesignSystem/Primitive";
  import { RemoteHelperHint, Tooltip } from "../../DesignSystem/Component";
  import { dismissRemoteHelperHint, settings } from "../../src/session.ts";

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
    border-radius: 8px;
    background: var(--color-background);
    border: 1px solid var(--color-foreground-level-3);
    box-shadow: var(--elevation-medium);
    padding: 1rem;
    width: 22.8rem;
  }

  p {
    color: var(--color-foreground-level-6);
    user-select: none;
  }
</style>

<svelte:window on:click={clickOutside} />

<div class="clone-dropdown" hidden={!expanded} bind:this={dropdown}>
  <p style="margin-bottom: 0.5rem;">
    Checkout a working copy to your local disk
  </p>

  <Input.Directory
    style="margin-bottom: 0.5rem;"
    placeholder="~/path/to/folder"
    buttonVariant="outline"
    bind:path={checkoutDirectoryPath} />

  {#if $settings.appearance.hints.showRemoteHelper}
    <RemoteHelperHint on:hide={dismissRemoteHelperHint} />
  {/if}

  <Tooltip
    value={!checkoutDirectoryPath ? 'Please select a folder' : null}
    position="bottom">
    <Button
      dataCy="checkout-button"
      on:click={() => {
        dispatch('checkout', { checkoutDirectoryPath: checkoutDirectoryPath });
        toggleDropdown();
      }}
      disabled={!checkoutDirectoryPath}
      variant="secondary"
      style="margin-top: 1rem; width: 100%; display: block; text-align: center;">
      Checkout
    </Button>
  </Tooltip>
</div>

<Button
  variant="transparent"
  icon={Icon.Copy}
  on:click={toggleDropdown}
  dataCy="checkout-modal-toggle">
  Checkout
</Button>
