<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Button from "design-system/Button.svelte";
  import Overlay from "design-system/Overlay.svelte";
  import RevisionIcon from "design-system/icons/Revision.svelte";

  import Copyable from "ui/App/SharedComponents/Copyable.svelte";

  export let expanded = false;

  const hide = (): void => {
    expanded = false;
  };

  const toggleDropdown = (): void => {
    expanded = !expanded;
  };
</script>

<style>
  .request-dropdown {
    margin-top: 3rem;
    right: 0;
    position: absolute;
    border-radius: 1rem;
    background: var(--color-background);
    box-shadow: var(--color-shadows);
    padding: 1rem;
    width: 25rem;
    color: var(--color-foreground-level-6);
  }

  .command-line {
    color: var(--color-foreground-level-6);
    overflow-x: scroll;
    padding: 0.5rem 0.5rem 0.5rem 0.25rem;
  }
</style>

<Overlay {expanded} on:hide={hide} style="position: relative;">
  <div class="request-dropdown" hidden={!expanded}>
    To create a patch in your working copy, check out the branch that contains
    the changes and run the following commands:
    <Copyable name="commands" on:copy={hide}>
      <pre
        class="typo-text-small-mono command-line">{`upstream patch create\nrad sync`}</pre>
    </Copyable>
  </div>

  <Button
    variant="transparent"
    icon={RevisionIcon}
    on:click={toggleDropdown}
    dataCy="patch-modal-toggle">
    New patch
  </Button>
</Overlay>
