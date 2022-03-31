<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import ArrowBoxUpRightIcon from "design-system/icons/ArrowBoxUpRight.svelte";
  import Copyable from "ui/App/SharedComponents/Copyable.svelte";
  import ForkIcon from "design-system/icons/Fork.svelte";

  import Button from "design-system/Button.svelte";
  import Overlay from "design-system/Overlay.svelte";

  export let projectUrn: string;
  // Whether this button should be displayed as a "Fork" button.
  export let fork: boolean = false;

  let expanded = false;

  const caption = fork ? "Fork" : "Checkout";
  const helpText = fork
    ? "To fork this project and checkout a working copy, run the following command in your terminal:"
    : "To checkout a working copy of this project, run the following command in your terminal:";

  let copyable: Copyable;
  const hide = (): boolean => (expanded = false);
  const toggleDropdown = (): void => {
    expanded = !expanded;
  };

  const copy = (): void => {
    copyable.copy();
    toggleDropdown();
  };
  $: instructions = [`rad checkout ${projectUrn}`].join("\n");
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

  .instructions {
    color: var(--color-foreground-level-6);
    overflow-x: scroll;
    padding: 0.5rem 0.5rem 0.5rem 0.25rem;
    white-space: pre;
  }
</style>

<Overlay {expanded} on:hide={hide} style="position: relative;">
  <div class="clone-dropdown" hidden={!expanded}>
    <p style="margin-bottom: 0.5rem;">{helpText}</p>

    {#if fork}
      <p>
        Your fork will be published under your identity, and visible to the
        network.
      </p>
    {/if}

    <Copyable name="command" bind:this={copyable}>
      <p class="typo-text-small-mono instructions">{instructions}</p>
    </Copyable>

    <Button
      style="display: block; margin: 1rem auto 0; width: 100%;"
      on:click={copy}>
      Copy
    </Button>
  </div>

  <Button
    variant="transparent"
    icon={fork ? ForkIcon : ArrowBoxUpRightIcon}
    on:click={toggleDropdown}
    dataCy="checkout-modal-toggle">
    {caption}
  </Button>
</Overlay>
