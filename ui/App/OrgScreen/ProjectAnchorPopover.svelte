<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { fade } from "svelte/transition";

  import type * as project from "ui/src/project";

  import * as router from "ui/src/router";
  import * as format from "ui/src/format";

  import { Avatar, Hoverable, Icon, Identifier } from "ui/DesignSystem";
  import TransactionHash from "ui/App/TransactionHash.svelte";

  export let anchor: project.Anchor;
  export let replicated: boolean = false;

  let hover: boolean;

  const openCommit = () => {
    router.push({
      type: "project",
      activeView: { type: "commit", commitHash: anchor.commitHash },
      urn: anchor.projectId,
    });
  };

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
  .container {
    position: absolute;
  }
  .modal {
    position: absolute;
    max-width: 23rem;
    border-radius: 0.5rem;
    background: var(--color-background);
    box-shadow: var(--color-shadows);
    top: -1rem;
    left: 2rem;
    z-index: 1;
  }
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
  .meta:last-of-type {
    padding-bottom: 1rem;
  }
  .org-domain {
    color: var(--color-foreground-level-6);
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }
</style>

<Hoverable bind:hovering={hover} style="display: inline-flex;">
  <Icon.AnchorSmall style={`fill: var(${anchorColor});`} />

  <!-- The click handler is necessary to prevent the event to propagate to the
    underlying project list item and navigating to the project when a user
    clicks on a link within the anchor popover. -->
  <div class="container" on:click|stopPropagation>
    {#if hover}
      <div class="modal" out:fade|local={{ duration: 100, delay: 250 }}>
        <div class="header">
          <Icon.Anchor
            style={`fill: var(${anchorColor}); margin-right: 0.5rem;`} />

          {#if anchor.orgAddress}
            <p class="typo-text-bold" style={`color: var(${titleColor})`}>
              {title}
            </p>
            <Avatar
              size="small"
              style="margin: 0 0.5rem 0 0.5rem;"
              kind={anchor.registration?.avatar
                ? { type: "orgImage", url: anchor.registration.avatar }
                : { type: "orgEmoji", uniqueIdentifier: anchor.orgAddress }} />
            {#if anchor.registration?.domain}
              <p class="typo-text-bold org-domain">
                {anchor.registration?.domain}
              </p>
            {:else}
              <Identifier
                kind="ethAddress"
                value={anchor.orgAddress}
                showIcon={false} />
            {/if}
          {/if}
        </div>
        {#if anchor.type === "pending"}
          <div class="meta">
            <Icon.Pen style="margin-right: 0.5rem;" />
            <p class="typo-text-bold" style="margin-right: 0.5rem;">
              Signed by {anchor.confirmations} of {anchor.threshold}
            </p>
          </div>
        {/if}

        {#if anchor.type === "confirmed"}
          <div class="meta">
            <Icon.Ethereum style="margin-right: 0.5rem;" />
            <p class="typo-text-bold" style="margin-right: 0.5rem;">
              Transaction hash
            </p>
            <TransactionHash hash={anchor.transactionId} />
          </div>
        {/if}
        <div class="meta">
          <Icon.Commit style="margin-right: 0.5rem;" />
          <p class="typo-text-bold" style="margin-right: 0.5rem;">
            Commit hash
          </p>
          {#if replicated}
            <span class="typo-link" on:click={openCommit}>
              {format.shortCommitHash(anchor.commitHash)}
            </span>
          {:else}
            <Identifier
              style="display: inline-block;"
              kind="commitHash"
              value={anchor.commitHash} />
          {/if}
        </div>
      </div>
    {/if}
  </div>
</Hoverable>
