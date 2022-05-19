<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { isDelegate, Project } from "ui/src/project";
  import * as Patch from "ui/src/project/patch";
  import * as Session from "ui/src/session";

  import ArrowBoxUpRightIcon from "design-system/icons/ArrowBoxUpRight.svelte";
  import Button from "design-system/Button.svelte";
  import MergeIcon from "design-system/icons/Merge.svelte";

  import CommandModal from "ui/App/SharedComponents/CommandModal.svelte";

  const session = Session.unsealed();

  export let project: Project;
  export let patch: Patch.Patch;
</script>

<div style="display: flex; gap: 1rem;">
  <CommandModal
    dataCy="checkout-patch-modal-toggle"
    let:prop={toggleDropdown}
    command={[
      `upstream patch fetch ${Patch.handle(patch)}`,
      `git checkout ${Patch.TAG_PREFIX}${Patch.handle(patch)}`,
    ].join("\n")}
    description="To fetch and check out this patch in your working copy, run the following commands:">
    <Button
      variant="transparent"
      icon={ArrowBoxUpRightIcon}
      on:click={toggleDropdown}>Checkout patch</Button>
  </CommandModal>
  {#if isDelegate(session.identity.urn, project) && !patch.merged}
    <CommandModal
      dataCy="merge-patch-modal-toggle"
      let:prop={toggleDropdown}
      command={[
        `upstream patch fetch ${Patch.handle(patch)}`,
        `git merge ${Patch.TAG_PREFIX}${Patch.handle(patch)}`,
        `rad push`,
      ].join("\n")}
      description="To merge this patch and publish the changes, run these commands in your working copy:">
      <Button variant="transparent" icon={MergeIcon} on:click={toggleDropdown}
        >Merge patch</Button>
    </CommandModal>
  {/if}
</div>
