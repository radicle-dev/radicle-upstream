<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Project } from "ui/src/project";

  import UnresolvedAnchorList from "ui/Screen/Org/UnresolvedAnchorList.svelte";

  import * as router from "ui/src/router";
  import * as sess from "ui/src/session";
  import * as org from "ui/src/org";

  import { EmptyState, ProjectList } from "ui/DesignSystem";

  const session = sess.getUnsealedFromContext();

  export let address: string;
  export let anchors: org.OrgAnchors;
  export let ownerAddress: string;
  export let disableAnchorCreation = false;
  export let isMultiSig: boolean;

  const select = ({ detail: project }: { detail: Project }) => {
    router.push({
      type: "project",
      activeView: { type: "files" },
      urn: project.urn,
    });
  };
</script>

<style>
  .container {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
  }

  .pending {
    margin-bottom: 2rem;
  }

  .header {
    display: flex;
    padding: 0.5rem 3rem 0.5rem;
    width: 100%;
  }
</style>

<div class="container">
  {#if anchors.pendingResolved.length !== 0 || anchors.pendingUnresolved.length !== 0}
    <div class="pending">
      <div class="header">
        <p class="typo-text-bold">Pending</p>
        <p style="margin-left: .5rem; color: var(--color-foreground-level-6);">
          Not enough of the members have signed this anchor.
        </p>
      </div>
      <ProjectList
        projects={anchors.pendingResolved}
        userUrn={session.identity.urn}
        on:select={select} />
      <UnresolvedAnchorList anchors={anchors.pendingUnresolved} />
    </div>
  {/if}

  {#if anchors.confirmedResolved.length !== 0}
    {#if anchors.pendingResolved.length !== 0 || anchors.pendingUnresolved.length !== 0}
      <div class="header">
        <p class="typo-text-bold">Confirmed</p>
        <p style="margin-left: .5rem; color: var(--color-foreground-level-6);">
          These projects have been anchored and signed by the org.
        </p>
      </div>
    {/if}
    <ProjectList
      projects={anchors.confirmedResolved}
      userUrn={session.identity.urn}
      on:select={select} />
  {/if}

  {#if anchors.confirmedUnresolved.length !== 0}
    <div class="header">
      <p style="color: var(--color-foreground-level-6);">
        These anchored projects haven't been found in your network yet, try
        following them.
      </p>
    </div>
    <UnresolvedAnchorList anchors={anchors.confirmedUnresolved} />
  {/if}

  {#if anchors.pendingResolved.length === 0 && anchors.confirmedResolved.length === 0 && anchors.pendingUnresolved.length === 0 && anchors.confirmedUnresolved.length === 0}
    <EmptyState
      emoji="🪴"
      text="Get started by anchoring your organization’s first project."
      primaryActionText={isMultiSig
        ? "Anchor with Gnosis Safe"
        : "Anchor Project"}
      primaryActionDisabled={disableAnchorCreation}
      primaryActionTooltipMessage="Create or follow a project first"
      on:primaryAction={() => {
        org.openAnchorProjectModal(address, ownerAddress, isMultiSig);
      }} />
  {/if}
</div>
