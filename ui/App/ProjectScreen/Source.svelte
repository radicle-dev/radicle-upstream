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

  import { onDestroy } from "svelte";

  import { openPath } from "ui/src/ipc";
  import { unreachable } from "ui/src/unreachable";
  import {
    fetch,
    watchPatchUpdates,
    selectPath,
    selectRevision,
    store,
  } from "ui/src/screen/project/source";
  import * as error from "ui/src/error";
  import * as notification from "ui/src/notification";
  import * as proxy from "ui/src/proxy";
  import * as remote from "ui/src/remote";
  import * as router from "ui/src/router";
  import * as screen from "ui/src/screen";
  import * as wallet from "ui/src/wallet";

  import AnchorIcon from "design-system/icons/Anchor.svelte";
  import CommitIcon from "design-system/icons/Commit.svelte";
  import FileIcon from "design-system/icons/File.svelte";
  import RevisionIcon from "design-system/icons/Revision.svelte";

  import Loading from "design-system/Loading.svelte";

  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import RevisionSelector from "ui/App/SharedComponents/RevisionSelector.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import CheckoutButton from "./Source/CheckoutButton.svelte";
  import PatchButton from "./Source/PatchButton.svelte";

  import Anchors from "./Source/Anchors.svelte";
  import CommitTab from "./Source/Commit.svelte";
  import CommitsTab from "./Source/Commits.svelte";
  import FilesTab from "./Source/Code.svelte";
  import Patch from "./Source/Patch.svelte";
  import PatchList from "./Source/PatchList.svelte";

  export let project: Project;
  export let selectedPeer: User;
  export let isContributor: boolean;
  export let anchors: ConfirmedAnchor[];

  export let activeView: projectRoute.ProjectView;

  const tabs = (active: projectRoute.ProjectView, screen: Screen) => {
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

  $: patchesTabSelected = activeView.type === "patches";

  const onCheckout = async (
    { detail: { checkoutPath } }: { detail: { checkoutPath: string } },
    project: Project,
    peer: User
  ) => {
    await screen.withLock(async () => {
      try {
        const path = await proxy.client.project.checkout(project.urn, {
          path: checkoutPath,
          peerId: peer.identity.peerId,
        });

        notification.show({
          type: "info",
          message: `${project.metadata.name} checked out to ${path}`,
          actions: [
            {
              label: "Open folder",
              handler: () => {
                openPath(path);
              },
            },
          ],
        });
      } catch (err: unknown) {
        let message;
        if (err instanceof proxy.ResponseError) {
          message = `Checkout failed: ${err.message}`;
        } else {
          message = `Checkout failed`;
        }
        notification.showException(
          new error.Error({
            code: error.Code.ProjectCheckoutFailure,
            message,
            source: err,
          })
        );
      }
    });
  };

  const onSelectRevision = ({ detail: revision }: { detail: Branch | Tag }) => {
    selectRevision(revision);
  };

  const unwatchPatchUpdates = watchPatchUpdates();
  onDestroy(unwatchPatchUpdates);
  $: fetch(project, selectedPeer);

  $: if ($store.status === remote.Status.Error) {
    notification.showException($store.error);
  }
</script>

<style>
  .loading-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: calc(100vh - var(--bigheader-height));
  }
</style>

{#if $store.status === remote.Status.Success}
  <ActionBar>
    <div slot="left">
      <div style="display: flex">
        {#if !patchesTabSelected}
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
      {#if patchesTabSelected}
        <PatchButton />
      {:else}
        <CheckoutButton
          fork={!isContributor}
          on:checkout={ev => onCheckout(ev, project, selectedPeer)} />
      {/if}
    </div>
  </ActionBar>

  {#if activeView.type === "files"}
    <FilesTab />
  {:else if activeView.type === "commits"}
    <CommitsTab />
  {:else if activeView.type === "commit"}
    <CommitTab
      commitHash={activeView.commitHash}
      anchors={anchors.filter(anchor => {
        return (
          activeView.type === "commit" &&
          anchor.commitHash === activeView.commitHash
        );
      })} />
  {:else if activeView.type === "patches"}
    <PatchList
      project={$store.data.project}
      patches={$store.data.patches}
      filter={activeView.filter} />
  {:else if activeView.type === "patch"}
    <Patch {project} id={activeView.id} peerId={activeView.peerId} />
  {:else if activeView.type === "anchors"}
    <Anchors {anchors} />
  {:else}
    {unreachable(activeView)}
  {/if}
{:else if $store.status === remote.Status.Loading}
  <div class="loading-container">
    <Loading />
  </div>
{/if}
