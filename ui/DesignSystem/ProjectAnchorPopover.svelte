<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { fade } from "svelte/transition";

  import type * as project from "ui/src/project";

  import * as org from "ui/src/org";
  import * as radicleAvatar from "radicle-avatar";
  import * as router from "ui/src/router";
  import * as style from "ui/src/style";

  import Avatar from "./Avatar.svelte";
  import Hoverable from "./Hoverable.svelte";
  import Icon from "./Icon";

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

  const bindOpenEtherscan = (txId: string) => () => {
    org.openOnEtherscan(txId);
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
    width: 354px;
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
    padding: 0 46px 0 1rem;
    color: var(--color-primary);
  }
  .meta {
    display: flex;
    color: var(--color-foreground-level-6);
    margin: 1rem;
    align-items: center;
  }
  .org {
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
              variant="square"
              imageUrl={anchor.registration?.avatar || undefined}
              avatarFallback={radicleAvatar.generate(
                anchor.orgAddress,
                radicleAvatar.Usage.Any
              )} />
            <p
              class="typo-text-bold org"
              style="color: var(--color-foreground-level-6);overflow: ellipsed">
              {anchor.registration?.domain || anchor.orgAddress}
            </p>
          {/if}
        </div>
        {#if anchor.type === "pending"}
          <div class="meta">
            <Icon.Pen style="margin-right: 0.5rem;" />
            <p class="typo-text-small-bold" style="margin-right: 0.5rem;">
              Signed by {anchor.confirmations} of {anchor.threshold}
            </p>
          </div>
        {/if}

        {#if anchor.type === "confirmed"}
          <div class="meta">
            <Icon.Ethereum style="margin-right: 0.5rem;" />
            <p class="typo-text-small-bold" style="margin-right: 0.5rem;">
              Transaction hash
            </p>
            <p
              class="typo-text-small typo-link"
              on:click={bindOpenEtherscan(anchor.transactionId)}>
              {style.ellipsed(anchor.transactionId, 6)}↗
            </p>
          </div>
        {/if}
        <div class="meta">
          <Icon.Commit style="margin-right: 0.5rem;" />
          <p class="typo-text-small-bold" style="margin-right: 0.5rem;">
            Commit hash
          </p>
          {#if replicated}
            <p class="typo-text-small typo-link" on:click={openCommit}>
              {anchor.commitHash.slice(0, 7)}↗
            </p>
          {:else}
            <p class="typo-text-small">{anchor.commitHash.slice(0, 7)}</p>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</Hoverable>
