<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Project } from "ui/src/project";

  import UnresolvedAnchorList from "ui/Screen/Org/UnresolvedAnchorList.svelte";

  import type * as project from "ui/src/project";

  import * as router from "ui/src/router";
  import * as sess from "ui/src/session";
  import * as org from "ui/src/org";

  import { EmptyState, ProjectList } from "ui/DesignSystem";

  const session = sess.getUnsealedFromContext();

  export let address: string;
  export let anchoredProjects: project.Project[];
  export let unresolvedAnchors: project.Anchor[];
  export let gnosisSafeAddress: string;
  export let disableAnchorCreation = false;

  const select = ({ detail: project }: { detail: Project }) => {
    router.push({
      type: "project",
      activeView: { type: "files" },
      urn: project.urn,
    });
  };

  let pendingResolved: project.Project[];
  let confirmedResolved: project.Project[];
  let pendingUnresolved: project.Anchor[];
  let confirmedUnresolved: project.Anchor[];

  $: {
    pendingResolved = anchoredProjects.filter(
      p => p.anchor && p.anchor.type === "pending"
    );
    confirmedResolved = anchoredProjects.filter(
      p => p.anchor && p.anchor.type === "confirmed"
    );
    pendingUnresolved = unresolvedAnchors.filter(a => a.type === "pending");
    confirmedUnresolved = unresolvedAnchors.filter(a => a.type === "confirmed");
  }
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
    padding: 1.5rem 3rem 0.5rem;
    width: 100%;
  }
</style>

<div class="container">
  {#if pendingResolved.length !== 0 || pendingUnresolved.length !== 0}
    <div class="pending">
      <div class="header">
        <p class="typo-text-bold">Pending</p>
        <p style="margin-left: .5rem; color: var(--color-foreground-level-6);">
          Not enough of the members have signed this anchor.
        </p>
      </div>
      {#if pendingResolved.length !== 0}
        <ProjectList
          projects={pendingResolved}
          userUrn={session.identity.urn}
          on:select={select} />
      {:else if pendingUnresolved.length !== 0}
        <UnresolvedAnchorList anchors={pendingUnresolved} />
      {/if}
    </div>
  {/if}
  {#if confirmedResolved.length !== 0 || confirmedUnresolved.length !== 0}
    {#if confirmedResolved.length !== 0}
      <ProjectList
        projects={confirmedResolved}
        userUrn={session.identity.urn}
        on:select={select} />
    {/if}
    {#if confirmedUnresolved.length !== 0}
      <div class="header">
        <p style="color: var(--color-foreground-level-6);">
          These anchored projects haven't been found in your network yet, try
          following them.
        </p>
      </div>
      <UnresolvedAnchorList anchors={confirmedUnresolved} />
    {/if}
  {:else}
    <EmptyState
      emoji="🪴"
      text="Get started by anchoring your organization’s first project."
      primaryActionText="Anchor with Gnosis Safe"
      primaryActionDisabled={disableAnchorCreation}
      primaryActionTooltipMessage="Create or follow a project first"
      on:primaryAction={() => {
        org.openAnchorProjectModal(address, gnosisSafeAddress);
      }} />
  {/if}
</div>
