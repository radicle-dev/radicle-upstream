<script>
  import { createEventDispatcher } from "svelte";

  import { Button, Input, Icon } from "../../../DesignSystem/Primitive";
  import { RemoteHelperHint, Tooltip } from "../../../DesignSystem/Component";
  import Overlay from "../../../DesignSystem/Component/Overlay.svelte";

  import { dismissRemoteHelperHint, settings } from "../../../src/session.ts";

  // Whether this button should be displayed as a "Fork" button.
  export let fork = false;

  let checkoutPath;
  let expanded = false;

  const caption = fork ? "Fork" : "Checkout";
  const helpText = fork
    ? "Fork this project and checkout a working copy."
    : "Checkout a working copy to your local disk.";

  const dispatch = createEventDispatcher();
  const hide = () => (expanded = false);
  const toggleDropdown = () => {
    expanded = !expanded;
  };
</script>

<style>
  .clone-dropdown {
    margin-top: 3rem;
    right: 0;
    position: absolute;
    border-radius: 1rem;
    background: var(--color-background);
    box-shadow: var(--color-shadows);
    padding: 1rem;
    width: 25rem;
  }

  p {
    color: var(--color-foreground-level-6);
    user-select: none;
  }
</style>

<Overlay {expanded} on:hide={hide} style="position: relative;">
  <div class="clone-dropdown" hidden={!expanded}>
    <p style="margin-bottom: 0.5rem;">{helpText}</p>

    <Input.Directory
      style="margin-bottom: 0.5rem;"
      placeholder="~/path/to/folder"
      buttonVariant="outline"
      bind:path={checkoutPath} />

    {#if fork}
      <p>
        Your fork will be published under your identity, and visible to the
        network.
      </p>
    {/if}

    {#if $settings.appearance.hints.showRemoteHelper}
      <RemoteHelperHint on:hide={dismissRemoteHelperHint} />
    {/if}

    <Tooltip
      value={!checkoutPath ? "Please select a folder" : ""}
      position="bottom">
      <Button
        dataCy="checkout-button"
        on:click={() => {
          dispatch("checkout", { checkoutPath: checkoutPath });
          toggleDropdown();
        }}
        disabled={!checkoutPath}
        style="margin-top: 1rem; width: 100%; display: block; text-align: center;">
        {caption}
      </Button>
    </Tooltip>
  </div>

  <Button
    variant="transparent"
    icon={fork ? Icon.Fork : Icon.ArrowBoxUpRight}
    on:click={toggleDropdown}
    dataCy="checkout-modal-toggle">
    {caption}
  </Button>
</Overlay>
