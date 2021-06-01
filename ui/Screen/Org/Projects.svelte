<script lang="typescript">
  import type { Project } from "ui/src/project";

  import ModalAnchorProject from "ui/Modal/Org/AnchorProject.svelte";
  import AnchorList from "ui/Screen/Org/AnchorList.svelte";

  import * as modal from "ui/src/modal";
  import * as org from "ui/src/org";
  import * as router from "ui/src/router";
  import * as sess from "ui/src/session";

  import { EmptyState, ProjectList } from "ui/DesignSystem/Component";
  import { Variant as IllustrationVariant } from "ui/src/illustration";

  const orgProjectTabStore = org.orgProjectTabStore;
  const orgScreenStore = org.orgScreenStore;
  const session = sess.getUnsealedFromContext();

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
  {#if $orgProjectTabStore.anchoredProjects.length !== 0 || $orgProjectTabStore.unresolvedAnchors.length !== 0}
    <ProjectList
      projects={$orgProjectTabStore.anchoredProjects}
      orgAddress={$orgScreenStore ? $orgScreenStore.orgAddress : ""}
      userUrn={session.identity.urn}
      on:select={select} />

    <AnchorList
      anchors={$orgProjectTabStore.unresolvedAnchors}
      orgAddress={$orgScreenStore ? $orgScreenStore.orgAddress : ""} />
  {:else}
    <EmptyState
      illustration={IllustrationVariant.Plant}
      text="Get started by anchoring your organization’s first project with the radicle gnosis safe app."
      primaryActionText="Anchor with Gnosis Safe"
      on:primaryAction={() => {
        modal.toggle(ModalAnchorProject);
      }} />
  {/if}
</div>
