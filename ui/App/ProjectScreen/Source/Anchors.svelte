<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as project from "ui/src/project";

  import dayjs from "dayjs";
  import * as zod from "zod";

  import * as browserStore from "ui/src/browserStore";
  import * as org from "ui/src/org";
  import * as router from "ui/src/router";

  import { Icon, IdentifierLink, format } from "ui/DesignSystem";
  import AnchorCard from "./AnchorCard.svelte";
  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";
  import OrgIdentity from "ui/App/SharedComponents/OrgIdentity.svelte";

  export let anchors: project.ConfirmedAnchor[];

  $: latest = anchors.slice(-1)[0];
  $: oldAnchors = anchors.slice(0, -1).reverse();

  const openCommit = (commitHash: string, projectId: string) => {
    router.push({
      type: "project",
      params: {
        activeView: { type: "commit", commitHash },
        urn: projectId,
      },
    });
  };

  const isAnchorHintVisible = browserStore.create<boolean>(
    "radicle.isAnchorHintVisible",
    true,
    zod.boolean()
  );
</script>

<style>
  .container {
    margin: 0 auto 6rem;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    padding: 2rem var(--content-padding) 0;
  }

  .banner {
    display: flex;
    padding: 0 1rem 0 1rem;
    align-items: center;
    border-radius: 0.5rem;
    background-color: var(--color-primary-level-1);
    color: var(--color-primary);
    height: 3rem;
    margin-bottom: 1rem;
  }

  .dismiss {
    margin-left: auto;
  }

  .timestamp {
    color: var(--color-foreground-level-5);
    margin-left: auto;
  }

  .anchor-list {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
    margin-top: 1rem;
  }

  .anchor {
    padding: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .anchor:not(:last-of-type) {
    border-bottom: 1px solid var(--color-foreground-level-2);
  }

  .left {
    display: flex;
    gap: 1rem;
  }
</style>

<div class="container">
  {#if $isAnchorHintVisible}
    <div class="banner">
      <Icon.InfoCircle
        style="fill: var(--color-primary); margin-right: 0.25rem;" />
      <div>
        Anchors are commits from this project anchored on the Ethereum network,
        via <a class="typo-link" href="https://gnosis-safe.io/">Gnosis Safe</a> or
        your wallet.
      </div>
      <div
        class="dismiss typo-link"
        on:click={() => {
          $isAnchorHintVisible = false;
        }}>
        Dismiss
      </div>
    </div>
  {/if}

  {#if latest}
    <AnchorCard anchor={latest} />
  {/if}

  {#if oldAnchors.length > 0}
    <div class="anchor-list">
      {#each oldAnchors as anchor}
        <div class="anchor">
          <div class="left">
            <OrgIdentity
              registration={anchor.registration}
              orgAddress={anchor.orgAddress} />
            <IdentifierLink
              showIcon={true}
              params={{
                type: "commitHash",
                hash: anchor.commitHash,
                onClick: () => {
                  openCommit(anchor.commitHash, anchor.projectId);
                },
              }} />
            <IdentifierLink
              showIcon={true}
              params={{
                type: "transactionHash",
                hash: anchor.transactionId,
                url: org.etherscanUrl(anchor.transactionId),
              }} />
          </div>
          <div class="timestamp">
            {dayjs
              .unix(anchor.timestamp)
              .format(format.TRANSACTION_TIMESTAMP_FORMAT)}
          </div>
        </div>
      {/each}
    </div>
  {/if}

  {#if latest === undefined && oldAnchors.length === 0}
    <EmptyState
      emoji="🚣"
      text="This project doesn't have any anchors yet. To anchor it, go to one of your orgs and click “Anchor project”." />
  {/if}
</div>
