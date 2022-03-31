<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import PlusIcon from "design-system/icons/Plus.svelte";

  import Button from "design-system/Button.svelte";
  import Overlay from "design-system/Overlay.svelte";

  import Copyable from "ui/App/SharedComponents/Copyable.svelte";

  let expanded = false;

  const hide = (): void => {
    expanded = false;
  };

  const toggleDropdown = (): void => {
    expanded = !expanded;
  };

  $: instructions = [`rad init`].join("\n");
</script>

<style>
  .dropdown {
    margin-top: 3rem;
    right: 0;
    position: absolute;
    z-index: 1;
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

<Overlay {expanded} on:hide={hide}>
  <div class="dropdown" hidden={!expanded}>
    <p style="margin-bottom: 0.5rem;">
      To initialize a Radicle project, run the following command in an existing
      repository:
    </p>
    <Copyable name="command" on:copy={toggleDropdown}>
      <p class="typo-text-small-mono instructions">{instructions}</p>
    </Copyable>
  </div>

  <Button variant="outline" icon={PlusIcon} on:click={toggleDropdown}>
    New project
  </Button>
</Overlay>
