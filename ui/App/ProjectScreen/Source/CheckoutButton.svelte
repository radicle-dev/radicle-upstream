<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import ArrowBoxUpRightIcon from "design-system/icons/ArrowBoxUpRight.svelte";
  import ForkIcon from "design-system/icons/Fork.svelte";

  import Button from "design-system/Button.svelte";
  import Overlay from "design-system/Overlay.svelte";
  import Tooltip from "design-system/Tooltip.svelte";

  import DirectoryInput from "ui/App/SharedComponents/DirectoryInput.svelte";
  import RemoteHelperHint from "ui/App/SharedComponents/RemoteHelperHint.svelte";

  // Whether this button should be displayed as a "Fork" button.
  export let fork: boolean = false;

  let checkoutPath: string;
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

    <DirectoryInput
      style="margin-bottom: 0.5rem;"
      placeholder="~/path/to/directory"
      bind:path={checkoutPath} />

    {#if fork}
      <p>
        Your fork will be published under your identity, and visible to the
        network.
      </p>
    {/if}

    <RemoteHelperHint />

    <Tooltip
      value={!checkoutPath ? "First choose a directory for this project" : ""}
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
    icon={fork ? ForkIcon : ArrowBoxUpRightIcon}
    on:click={toggleDropdown}
    dataCy="checkout-modal-toggle">
    {caption}
  </Button>
</Overlay>
