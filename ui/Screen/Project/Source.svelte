<script lang="typescript">
  import { onDestroy } from "svelte";

  import * as error from "../../src/error";
  import { openPath } from "../../src/ipc";
  import * as notification from "../../src/notification";
  import * as proxy from "../../src/proxy";
  import { unreachable } from "ui/src/unreachable";
  import * as router from "ui/src/router";
  import * as remote from "ui/src/remote";
  import type { Project, User } from "../../src/project";
  import {
    fetch,
    watchPatchUpdates,
    selectPath,
    selectRevision,
    store,
  } from "../../src/screen/project/source";
  import type { Screen } from "ui/src/screen/project/source";
  import type { Branch, Tag } from "../../src/source";
  import * as screen from "../../src/screen";

  import ActionBar from "../../DesignSystem/Component/ActionBar.svelte";
  import TabBar from "ui/DesignSystem/Component/TabBar.svelte";
  import RevisionSelector from "../../DesignSystem/Component/SourceBrowser/RevisionSelector.svelte";

  import CheckoutButton from "./Source/CheckoutButton.svelte";
  import PatchButton from "./Source/PatchButton.svelte";

  import FilesTab from "ui/Screen/Project/Source/Code.svelte";
  import CommitsTab from "ui/Screen/Project/Source/Commits.svelte";
  import CommitTab from "ui/Screen/Project/Source/Commit.svelte";
  import PatchList from "./Source/PatchList.svelte";
  import Patch from "./Source/Patch.svelte";

  import { Icon } from "ui/DesignSystem/Primitive";
  export let project: Project;
  export let selectedPeer: User;
  export let isContributor: boolean;

  export let activeView: router.ProjectView;

  const tabs = (active: router.ProjectView, screen: Screen) => {
    return [
      {
        title: "Files",
        active: active.type === "files",
        icon: Icon.File,
        onClick: () => {
          if (activeView.type === "files") {
            selectPath("");
          } else {
            router.push({
              type: "project",
              urn: project.urn,
              activeView: { type: "files" },
            });
          }
        },
      },
      {
        title: "Commits",
        active: active.type === "commits",
        icon: Icon.Commit,
        counter: screen.history.stats.commits,
        onClick: () => {
          router.push({
            type: "project",
            urn: project.urn,
            activeView: { type: "commits" },
          });
        },
      },
      {
        title: "Patches",
        active: active.type === "patches",
        icon: Icon.Commit,
        counter: screen.patches.filter(patch => !patch.merged).length,
        onClick: () => {
          router.push({
            type: "project",
            urn: project.urn,
            activeView: { type: "patches", filter: "open" },
          });
        },
      },
    ];
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

        notification.info({
          message: `${project.metadata.name} checked out to ${path}`,
          showIcon: true,
          actions: [
            {
              label: "Open folder",
              handler: () => {
                openPath(path);
              },
            },
          ],
        });
      } catch (err) {
        error.show(
          new error.Error({
            code: error.Code.ProjectCheckoutFailure,
            message: `Checkout failed: ${err.message}`,
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
    error.show($store.error);
  }
</script>

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
    <CommitTab commitHash={activeView.commitHash} />
  {:else if activeView.type === "patches"}
    <PatchList
      project={$store.data.project}
      patches={$store.data.patches}
      filter={activeView.filter} />
  {:else if activeView.type === "patch"}
    <Patch {project} id={activeView.id} peerId={activeView.peerId} />
  {:else}
    {unreachable(activeView)}
  {/if}
{/if}
