<script>
  import { createEventDispatcher } from "svelte";

  import { Button, Input, Icon } from "../../DesignSystem/Primitive";
  import { RemoteHelperHint, Tooltip } from "../../DesignSystem/Component";
  import { dismissRemoteHelperHint, settings } from "../../src/session.ts";
  import Overlay from "../../DesignSystem/Component/Overlay.svelte";

  const dispatch = createEventDispatcher();

  let expanded = false;

  const toggleDropdown = () => {
    expanded = !expanded;
  };

  const hide = () => (expanded = false);

  let checkoutDirectoryPath;
</script>

<style>
  .clone-dropdown {
    margin-top: 3rem;
    right: 2rem;
    position: absolute;
    border-radius: 8px;
    background: var(--color-background);
    border: 1px solid var(--color-foreground-level-3);
    box-shadow: var(--elevation-medium);
    padding: 1rem;
    width: 25rem;
  }

  p {
    color: var(--color-foreground-level-6);
    user-select: none;
  }
</style>

<Overlay {expanded} on:hide={hide}>
  <div class="clone-dropdown" hidden={!expanded}>
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
      value={!checkoutDirectoryPath ? 'Please select a folder' : ''}
      position="bottom">
      <Button
        dataCy="checkout-button"
        on:click={() => {
          dispatch('checkout', {
            checkoutDirectoryPath: checkoutDirectoryPath,
          });
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
    icon={Icon.ArrowBoxUpRight}
    on:click={toggleDropdown}
    dataCy="checkout-modal-toggle">
    Checkout
  </Button>
</Overlay>
