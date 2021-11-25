<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as project from "ui/src/project";

  import * as org from "ui/src/org";
  import * as router from "ui/src/router";

  import AnchorIcon from "design-system/icons/Anchor.svelte";
  import AnchorSmallIcon from "design-system/icons/AnchorSmall.svelte";
  import CommitIcon from "design-system/icons/Commit.svelte";
  import EthereumIcon from "design-system/icons/Ethereum.svelte";
  import PenIcon from "design-system/icons/Pen.svelte";

  import Hovercard from "design-system/Hovercard.svelte";
  import IdentifierLink from "design-system/IdentifierLink.svelte";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import OrgIdentity from "ui/App/SharedComponents/OrgIdentity.svelte";

  export let anchor: project.Anchor;
  export let replicated: boolean = false;

  function openCommit(): void {
    router.push({
      type: "project",
      params: {
        activeView: { type: "commit", commitHash: anchor.commitHash },
        urn: anchor.projectId,
      },
    });
  }

  $: anchorColor =
    anchor.type === "confirmed"
      ? "--color-primary"
      : "--color-foreground-level-5";
  $: title = anchor.type === "confirmed" ? "Anchored in" : "Pending in";
  $: titleColor =
    anchor.type === "confirmed"
      ? "--color-primary"
      : "--color-foreground-level-5";
</script>

<style>
  .header {
    border-bottom: 1px solid var(--color-foreground-level-2);
    height: 3.5rem;
    align-items: center;
    display: flex;
    color: var(--color-primary);
    padding-left: 1rem;
    padding-right: 1rem;
  }
  .meta {
    display: flex;
    color: var(--color-foreground-level-6);
    padding: 1rem 1rem 0 1rem;
    align-items: center;
  }
  .meta:last-child {
    padding-bottom: 1rem;
  }
</style>

<Hovercard style="display: inline-flex;">
  <svelte:fragment slot="trigger">
    <AnchorSmallIcon style={`fill: var(${anchorColor});`} />
  </svelte:fragment>

  <svelte:fragment slot="card">
    <div class="header">
      <AnchorIcon style={`fill: var(${anchorColor}); margin-right: 0.5rem;`} />

      {#if anchor.orgAddress}
        <span
          class="typo-text-bold"
          style={`color: var(${titleColor}); margin-right: 0.5rem;`}>
          {title}
        </span>

        <OrgIdentity
          orgAddress={anchor.orgAddress}
          registration={anchor.registration} />
      {/if}
    </div>
    {#if anchor.type === "pending"}
      <div class="meta">
        <PenIcon style="margin-right: 0.5rem;" />
        <span class="typo-text-bold" style="margin-right: 0.5rem;">
          Signed by {anchor.confirmations} of {anchor.threshold}
        </span>
      </div>
    {/if}

    {#if anchor.type === "confirmed"}
      <div class="meta">
        <EthereumIcon style="margin-right: 0.5rem;" />
        <span class="typo-text-bold" style="margin-right: 0.5rem;">
          Transaction hash
        </span>
        <IdentifierLink
          params={{
            type: "transactionHash",
            url: org.etherscanUrl(anchor.transactionId),
            hash: anchor.transactionId,
          }} />
      </div>
    {/if}
    <div class="meta">
      <CommitIcon style="margin-right: 0.5rem;" />
      <span class="typo-text-bold" style="margin-right: 0.5rem;">
        Commit hash
      </span>
      {#if replicated}
        <IdentifierLink
          params={{
            type: "commitHash",
            hash: anchor.commitHash,
            onClick: openCommit,
          }} />
      {:else}
        <CopyableIdentifier
          style="display: inline-block;"
          kind="commitHash"
          value={anchor.commitHash} />
      {/if}
    </div>
  </svelte:fragment>
</Hovercard>
