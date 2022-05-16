<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Project, User, ConfirmedAnchor } from "ui/src/project";
  import type { Screen } from "ui/src/screen/project/source";
  import type { Branch, Tag } from "ui/src/source";
  import type * as projectRoute from "./route";

  import { unreachable } from "ui/src/unreachable";
  import {
    fetch,
    selectPath,
    selectRevision,
    store,
  } from "ui/src/screen/project/source";
  import { isDelegate } from "ui/src/project";
  import * as Patch from "ui/src/project/patch";
  import * as Session from "ui/src/session";
  import * as notification from "ui/src/notification";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";
  import * as wallet from "ui/src/wallet";
  import { patchStatusStore } from "ui/App/ProjectScreen/Source/Patch.svelte";

  import AnchorIcon from "design-system/icons/Anchor.svelte";
  import ArrowBoxUpRightIcon from "design-system/icons/ArrowBoxUpRight.svelte";
  import Button from "design-system/Button.svelte";
  import CommitIcon from "design-system/icons/Commit.svelte";
  import FileIcon from "design-system/icons/File.svelte";
  import ForkIcon from "design-system/icons/Fork.svelte";
  import MergeIcon from "design-system/icons/Merge.svelte";
  import RevisionIcon from "design-system/icons/Revision.svelte";

  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import CommandModal from "ui/App/SharedComponents/CommandModal.svelte";
  import Loading from "ui/App/SharedComponents/Loading.svelte";
  import RevisionSelector from "ui/App/SharedComponents/RevisionSelector.svelte";
  import TabBar, { Tab } from "ui/App/ScreenLayout/TabBar.svelte";

  import History from "./Source/SourceBrowser/History.svelte";

  import AnchorsTab from "./Source/Anchors.svelte";
  import CommitTab from "./Source/Commit.svelte";
  import FilesTab from "./Source/Code.svelte";
  import PatchListTab from "./Source/PatchList.svelte";
  import PatchTab from "./Source/Patch.svelte";

  export let project: Project;
  export let selectedPeer: User;
  export let isContributor: boolean;
  export let anchors: ConfirmedAnchor[];

  export let activeView: projectRoute.ProjectView;

  const session = Session.unsealed();

  const tabs = (active: projectRoute.ProjectView, screen: Screen): Tab[] => {
    const items = [
      {
        title: "Files",
        active: active.type === "files",
        icon: FileIcon,
        onClick: () => {
          if (activeView.type === "files") {
            selectPath("");
          } else {
            router.push({
              type: "project",
              params: {
                urn: project.urn,
                activeView: { type: "files" },
              },
            });
          }
        },
      },
      {
        title: "Commits",
        active: active.type === "commits",
        icon: CommitIcon,
        counter: screen.history.stats.commits,
        onClick: () => {
          router.push({
            type: "project",
            params: {
              urn: project.urn,
              activeView: { type: "commits" },
            },
          });
        },
      },
      {
        title: "Patches",
        active: active.type === "patches",
        icon: RevisionIcon,
        counter: screen.patches.filter(patch => !patch.merged).length,
        onClick: () => {
          router.push({
            type: "project",
            params: {
              urn: project.urn,
              activeView: { type: "patches", filter: "open" },
            },
          });
        },
      },
    ];

    if (wallet.isConnected()) {
      return [
        ...items,
        {
          title: "Anchors",
          active: active.type === "anchors",
          icon: AnchorIcon,
          counter: anchors.length,
          onClick: () => {
            router.push({
              type: "project",
              params: {
                urn: project.urn,
                activeView: { type: "anchors" },
              },
            });
          },
        },
      ];
    } else {
      return items;
    }
  };

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
    <div slot="left">
      <div style="display: flex">
        {#if activeView.type === "files" || activeView.type === "commits"}
          <RevisionSelector
            style="width: 18rem; margin-right: 2rem;"
            loading={$store.data.selectedRevision.request !== null}
            on:select={onSelectRevision}
            selected={$store.data.selectedRevision.selected}
            defaultBranch={project.metadata.defaultBranch}
            revisions={$store.data.revisions} />
        {/if}

        <TabBar tabs={tabs(activeView, $store.data)} />
      </div>
    </div>
    <div slot="right">
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
          <div style="display: flex; gap: 1rem;">
            <CommandModal
              dataCy="checkout-patch-modal-toggle"
              let:prop={toggleDropdown}
              command={[
                `upstream patch fetch ${Patch.handle($patchStatusStore.patch)}`,
                `git checkout ${Patch.TAG_PREFIX}${Patch.handle(
                  $patchStatusStore.patch
                )}`,
              ].join("\n")}
              description="To fetch and check out this patch in your working copy, run the following commands:">
              <Button
                variant="transparent"
                icon={ArrowBoxUpRightIcon}
                on:click={toggleDropdown}>Checkout patch</Button>
            </CommandModal>
            {#if isDelegate(session.identity.urn, project) && !$patchStatusStore.patch.merged}
              <CommandModal
                dataCy="merge-patch-modal-toggle"
                let:prop={toggleDropdown}
                command={[
                  `upstream patch fetch ${Patch.handle(
                    $patchStatusStore.patch
                  )}`,
                  `git merge ${Patch.TAG_PREFIX}${Patch.handle(
                    $patchStatusStore.patch
                  )}`,
                  `rad push`,
                ].join("\n")}
                description="To merge this patch and publish the changes, run these commands in your working copy:">
                <Button
                  variant="transparent"
                  icon={MergeIcon}
                  on:click={toggleDropdown}>Merge patch</Button>
              </CommandModal>
            {/if}
          </div>
        {/if}
      {:else if activeView.type === "files" || activeView.type === "commits" || activeView.type === "commit" || activeView.type === "anchors"}
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
      {:else}
        {unreachable(activeView)}
      {/if}
    </div>
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
      patches={$store.data.patches}
      filter={activeView.filter} />
  {:else if activeView.type === "patch"}
    <PatchTab {project} id={activeView.id} peerId={activeView.peerId} />
  {:else if activeView.type === "anchors"}
    <AnchorsTab {anchors} />
  {:else}
    {unreachable(activeView)}
  {/if}
{:else if $store.status === remote.Status.Loading}
  <Loading style="height: calc(100vh - var(--bigheader-height));" />
{/if}
