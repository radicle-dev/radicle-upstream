<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as project from "ui/src/project";

  import dayjs from "dayjs";

  import * as org from "ui/src/org";
  import * as router from "ui/src/router";

  import * as format from "design-system/lib/format";

  import AnchorIcon from "design-system/icons/Anchor.svelte";
  import CommitIcon from "design-system/icons/Commit.svelte";
  import EthereumIcon from "design-system/icons/Ethereum.svelte";

  import IdentifierLink from "design-system/IdentifierLink.svelte";

  import OrgIdentity from "ui/App/SharedComponents/OrgIdentity.svelte";

  export let anchor: project.ConfirmedAnchor;
  export let showCommitHash: boolean = true;
  export let style: string | undefined = undefined;

  const openCommit = (commitHash: string, projectId: string) => {
    router.push({
      type: "project",
      params: {
        activeView: { type: "commit", commitHash },
        urn: projectId,
      },
    });
  };
</script>

<style>
  .anchor {
    background-color: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-2);
  }

  .header {
    border-bottom: 1px solid var(--color-foreground-level-2);
    min-height: 3.5rem;
    display: flex;
    align-items: center;
    padding: 0 1rem 0 1rem;
  }

  .timestamp {
    color: var(--color-foreground-level-5);
    margin-left: auto;
  }

  .metadata {
    padding: 1rem 1rem 0 1rem;
    display: flex;
    gap: 0.5rem;
    color: var(--color-foreground-level-6);
  }

  .metadata:last-of-type {
    padding-bottom: 1rem;
  }
</style>

<div class="anchor" {style}>
  <div class="header">
    <div style="display: flex;">
      <AnchorIcon />
      <span
        class="typo-text-bold"
        style="color: var(--color-foreground-level-6); margin: 0 0.5rem 0 0.5rem;">
        Anchored by
      </span>
      <OrgIdentity
        registration={anchor.registration}
        orgAddress={anchor.orgAddress} />
    </div>
    <span class="timestamp">
      {dayjs.unix(anchor.timestamp).format(format.TRANSACTION_TIMESTAMP_FORMAT)}
    </span>
  </div>
  {#if showCommitHash}
    <div class="metadata">
      <CommitIcon />
      <span class="typo-text-bold">Commit hash</span>
      <IdentifierLink
        params={{
          type: "commitHash",
          hash: anchor.commitHash,
          onClick: () => {
            openCommit(anchor.commitHash, anchor.projectId);
          },
        }} />
    </div>
  {/if}
  <div class="metadata">
    <EthereumIcon />
    <span class="typo-text-bold">Transaction hash</span>
    <IdentifierLink
      params={{
        type: "transactionHash",
        hash: anchor.transactionId,
        url: org.etherscanUrl(anchor.transactionId),
      }} />
  </div>
</div>
