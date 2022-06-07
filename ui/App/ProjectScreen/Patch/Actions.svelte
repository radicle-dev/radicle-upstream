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
  import * as Error from "ui/src/error";

  import ArrowBoxUpRightIcon from "design-system/icons/ArrowBoxUpRight.svelte";
  import Button from "design-system/Button.svelte";
  import MergeIcon from "design-system/icons/Merge.svelte";
  import CrossIcon from "design-system/icons/Cross.svelte";

  import CommandModal from "ui/App/SharedComponents/CommandModal.svelte";

  export let project: Project;
  export let patch: Patch.Patch;

  const session = Session.unsealed();

  let updateStatusInProgress = false;

  async function updateStatus(status: "open" | "closed") {
    updateStatusInProgress = true;
    try {
      await Patch.publishEvent(
        { projectUrn: project.urn, peerId: patch.peerId, name: patch.id },
        {
          type: "setStatus",
          data: {
            status,
          },
        }
      );
    } catch (err: unknown) {
      Error.showNotification(
        new Error.Error({
          message: "Failed to set patch status",
          details: {
            projectUrn: project.urn,
            peerId: patch.peerId,
            name: patch.id,
          },
          source: err,
        })
      );
    } finally {
      updateStatusInProgress = false;
    }
  }

  $: permissions = {
    close:
      (isDelegate(session.identity.urn, project) ||
        patch.peerId === session.identity.peerId) &&
      patch.status.current === "open",
    reopen:
      (isDelegate(session.identity.urn, project) ||
        patch.peerId === session.identity.peerId) &&
      patch.status.current === "closed",
    merge:
      isDelegate(session.identity.urn, project) &&
      patch.status.current === "open",
  };
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
  {#if permissions.close}
    <Button
      icon={CrossIcon}
      disabled={updateStatusInProgress}
      variant="transparent"
      on:click={() => updateStatus("closed")}>Close patch</Button>
  {/if}
  {#if permissions.reopen}
    <Button
      variant="transparent"
      disabled={updateStatusInProgress}
      on:click={() => updateStatus("open")}>Reopen patch</Button>
  {/if}
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
