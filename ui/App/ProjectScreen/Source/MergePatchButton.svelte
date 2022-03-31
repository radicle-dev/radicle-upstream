<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as Patch from "ui/src/project/patch";

  import MergeIcon from "design-system/icons/Merge.svelte";

  import Button from "design-system/Button.svelte";
  import Overlay from "design-system/Overlay.svelte";

  import Copyable from "ui/App/SharedComponents/Copyable.svelte";

  export let patch: Patch.Patch;

  let expanded = false;
  let copyable: Copyable;
  const hide = (): void => {
    expanded = false;
  };
  const toggleDropdown = (): void => {
    expanded = !expanded;
  };
  const copy = (): void => {
    copyable.copy();
    toggleDropdown();
  };

  $: patchHandle = Patch.handle(patch);
  $: instructions = [
    `upstream patch fetch ${patchHandle}`,
    `git merge ${Patch.TAG_PREFIX}${patchHandle}`,
    `rad push`,
  ].join("\n");
</script>

<style>
  .request-dropdown {
    margin-top: 3rem;
    right: 0;
    position: absolute;
    z-index: 1;
    border-radius: 1rem;
    background: var(--color-background);
    box-shadow: var(--color-shadows);
    padding: 1rem;
    width: 25rem;
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
  <div class="request-dropdown" hidden={!expanded}>
    <p style="margin-bottom: 0.5rem;">
      To merge this patch and publish the changes, run these commands in your
      working copy:
    </p>
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
    icon={MergeIcon}
    on:click={toggleDropdown}
    dataCy="merge-patch-modal-toggle">
    Merge
  </Button>
</Overlay>
