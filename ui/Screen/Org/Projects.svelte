<script lang="typescript">
  import type { Project } from "ui/src/project";

  import UnresolvedAnchorList from "ui/Screen/Org/UnresolvedAnchorList.svelte";

  import type * as project from "ui/src/project";

  import * as router from "ui/src/router";
  import * as sess from "ui/src/session";
  import * as org from "ui/src/org";

  import { EmptyState, ProjectList } from "ui/DesignSystem/Component";
  import { Variant as IllustrationVariant } from "ui/src/illustration";

  const session = sess.getUnsealedFromContext();

  export let address: string;
  export let anchoredProjects: project.Project[];
  export let unresolvedAnchors: org.ProjectAnchor[];
  export let gnosisSafeAddress: string;

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
</style>

<div class="container">
  {#if anchoredProjects.length !== 0 || unresolvedAnchors.length !== 0}
    <ProjectList
      projects={anchoredProjects}
      orgAddress={address}
      userUrn={session.identity.urn}
      on:select={select} />

    <UnresolvedAnchorList anchors={unresolvedAnchors} orgAddress={address} />
  {:else}
    <EmptyState
      illustration={IllustrationVariant.Plant}
      text="Get started by anchoring your organization’s first project with the radicle gnosis safe app."
      primaryActionText="Anchor with Gnosis Safe"
      on:primaryAction={() => {
        org.openAnchorProjectModal(address, gnosisSafeAddress);
      }} />
  {/if}
</div>
