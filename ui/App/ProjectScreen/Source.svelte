<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Project, User, ConfirmedAnchor } from "ui/src/project";
  import type { Branch, Tag } from "ui/src/source";
  import { Patch } from "ui/src/project/patch";
  import type * as projectRoute from "./route";

  import { unreachable } from "ui/src/unreachable";
  import { fetch, selectRevision, store } from "ui/src/screen/project/source";
  import * as notification from "ui/src/notification";
  import * as remote from "ui/src/remote";

  import ArrowBoxUpRightIcon from "design-system/icons/ArrowBoxUpRight.svelte";
  import Button from "design-system/Button.svelte";
  import ForkIcon from "design-system/icons/Fork.svelte";
  import RevisionIcon from "design-system/icons/Revision.svelte";

  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import CommandModal from "ui/App/SharedComponents/CommandModal.svelte";
  import Loading from "ui/App/SharedComponents/Loading.svelte";
  import RevisionSelector from "ui/App/SharedComponents/RevisionSelector.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import { makeTabs } from "./tabs";

  import History from "./Source/SourceBrowser/History.svelte";

  import AnchorsTab from "./Source/Anchors.svelte";
  import CommitTab from "./Source/Commit.svelte";
  import FilesTab from "./Source/Code.svelte";
  import PatchListTab from "./Source/PatchList.svelte";
  import PatchTab, { patchStatusStore } from "./Source/Patch.svelte";
  import PatchActions from "./Source/PatchActions.svelte";

  export let project: Project;
  export let selectedPeer: User;
  export let isContributor: boolean;
  export let anchors: ConfirmedAnchor[];
  export let patches: Patch[];
  export let activeView: projectRoute.ProjectView;

  const onSelectRevision = ({
    detail: revision,
  }: {
    detail: Branch | Tag;
  }): void => {
    selectRevision(revision);
  };

  $: fetch(project, selectedPeer);

  $: if ($store.status === remote.Status.Error) {
    notification.showException($store.error);
  }

  $: seedArg = project.seed ? `--seed ${new URL(project.seed).host}` : "";
</script>

<style>
  .commits-page {
    margin: 0 auto 6rem;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    padding: 2rem var(--content-padding) 0;
  }
</style>

{#if $store.status === remote.Status.Success}
  <ActionBar>
    {#if activeView.type === "files" || activeView.type === "commits"}
      <RevisionSelector
        style="width: 18rem; margin-right: 2rem;"
        loading={$store.data.selectedRevision.request !== null}
        on:select={onSelectRevision}
        selected={$store.data.selectedRevision.selected}
        defaultBranch={project.metadata.defaultBranch}
        revisions={$store.data.revisions} />
    {/if}

    <TabBar
      tabs={makeTabs({
        projectUrn: project.urn,
        activeViewType: activeView.type,
        commitCount: $store.data.history.stats.commits,
        patchCount: patches.filter(patch => patch.status.current === "open")
          .length,
      })} />

    <div style="margin-left: auto" />

    {#if activeView.type === "patches"}
      <CommandModal
        let:prop={toggleDropdown}
        command={"upstream patch create"}
        description="To create a patch in your working copy, check out the branch that contains the changes and run the following command:">
        <Button
          variant="transparent"
          icon={RevisionIcon}
          on:click={toggleDropdown}
          dataCy="patch-modal-toggle">Create patch</Button>
      </CommandModal>
    {:else if activeView.type === "patch"}
      {#if $patchStatusStore.type === "ok"}
        <PatchActions {project} patch={$patchStatusStore.patch} />
      {/if}
    {:else if activeView.type === "files"}
      {#if isContributor}
        <CommandModal
          let:prop={toggleDropdown}
          command={[
            `rad checkout ${project.urn} && \\`,
            `cd "${project.metadata.name}" && \\`,
            `rad sync ${seedArg}`,
          ].join("\n")}
          description="To checkout a working copy of this project, run the following command in your terminal:">
          <Button
            variant="transparent"
            icon={ArrowBoxUpRightIcon}
            on:click={toggleDropdown}>Checkout project</Button>
        </CommandModal>
      {:else}
        <CommandModal
          let:prop={toggleDropdown}
          command={[
            `rad checkout ${project.urn} && \\`,
            `cd "${project.metadata.name}" && \\`,
            `rad push ${seedArg} && \\`,
            `rad sync --self ${seedArg}`,
          ].join("\n")}
          description="To fork this project and checkout a working copy, run the following command in your terminal:">
          <Button
            variant="transparent"
            icon={ForkIcon}
            on:click={toggleDropdown}>
            Fork
          </Button>
        </CommandModal>
      {/if}
    {:else if activeView.type === "commits" || activeView.type === "commit" || activeView.type === "anchors"}
      <!-- Don't show any actions. -->
    {:else}
      {unreachable(activeView)}
    {/if}
  </ActionBar>

  {#if activeView.type === "files"}
    <FilesTab />
  {:else if activeView.type === "commits"}
    <div class="commits-page" data-cy="commits-page">
      <History projectUrn={project.urn} history={$store.data.history} />
    </div>
  {:else if activeView.type === "commit"}
    <CommitTab
      projectUrn={$store.data.project.urn}
      commitHash={activeView.commitHash}
      anchors={anchors.filter(anchor => {
        return (
          activeView.type === "commit" &&
          anchor.commitHash === activeView.commitHash
        );
      })} />
  {:else if activeView.type === "patches"}
    <PatchListTab
      project={$store.data.project}
      {patches}
      filter={activeView.filter} />
  {:else if activeView.type === "patch"}
    <PatchTab
      {project}
      id={activeView.id}
      peerId={activeView.peerId}
      view={activeView.view} />
  {:else if activeView.type === "anchors"}
    <AnchorsTab {anchors} />
  {:else}
    {unreachable(activeView)}
  {/if}
{:else if $store.status === remote.Status.Loading}
  <Loading style="height: calc(100vh - var(--bigheader-height));" />
{/if}
