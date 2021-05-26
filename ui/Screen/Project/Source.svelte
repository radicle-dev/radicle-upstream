<script lang="typescript">
  import * as error from "../../src/error";
  import { openPath } from "../../src/ipc";
  import * as notification from "../../src/notification";
  import * as proxy from "../../src/proxy";
  import * as router from "ui/src/router";
  import type { Project, User } from "../../src/project";
  import {
    fetch,
    selectPath,
    selectRevision,
    store,
  } from "../../src/screen/project/source";
  import type { Branch, Tag } from "../../src/source";
  import * as screen from "../../src/screen";

  import ActionBar from "../../DesignSystem/Component/ActionBar.svelte";
  import TabBar from "ui/DesignSystem/Component/TabBar.svelte";
  import Remote from "../../DesignSystem/Component/Remote.svelte";
  import RevisionSelector from "../../DesignSystem/Component/SourceBrowser/RevisionSelector.svelte";

  import CheckoutButton from "./Source/CheckoutButton.svelte";

  import FilesTab from "ui/Screen/Project/Source/Code.svelte";
  import CommitsTab from "ui/Screen/Project/Source/Commits.svelte";
  import CommitTab from "ui/Screen/Project/Source/Commit.svelte";

  import { Icon } from "ui/DesignSystem/Primitive";

  export let project: Project;
  export let selectedPeer: User;
  export let isContributor: boolean;

  export let activeView: router.ProjectView;

  const tabs = (active: router.ProjectView, commitCount: number) => {
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
        counter: commitCount,
        onClick: () => {
          router.push({
            type: "project",
            urn: project.urn,
            activeView: { type: "commits" },
          });
        },
      },
    ];
  };

  const onCheckout = async (
    { detail: { checkoutPath } }: { detail: { checkoutPath: string } },
    project: Project,
    peer: User
  ) => {
    try {
      screen.lock();
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
    } finally {
      screen.unlock();
    }
  };

  const onSelectRevision = ({ detail: revision }: { detail: Branch | Tag }) => {
    selectRevision(revision);
  };

  $: fetch(project, selectedPeer);
</script>

<style>
  .revision-selector-wrapper {
    width: 18rem;
    position: relative;
    margin-right: 2rem;
  }
</style>

<Remote {store} let:data={{ history, revisions, selectedRevision }}>
  <ActionBar>
    <div slot="left">
      <div style="display: flex">
        <div class="revision-selector-wrapper">
          <RevisionSelector
            loading={selectedRevision.request !== null}
            on:select={onSelectRevision}
            selected={selectedRevision.selected}
            defaultBranch={project.metadata.defaultBranch}
            {revisions} />
        </div>

        <TabBar tabs={tabs(activeView, history.stats.commits)} />
      </div>
    </div>
    <div slot="right">
      <CheckoutButton
        fork={!isContributor}
        on:checkout={ev => onCheckout(ev, project, selectedPeer)} />
    </div>
  </ActionBar>

  {#if activeView.type === "files"}
    <FilesTab />
  {:else if activeView.type === "commits"}
    <CommitsTab />
  {:else if activeView.type === "commit"}
    <CommitTab commitHash={activeView.commitHash} />
  {:else}
    {router.unreachable(activeView)}
  {/if}
</Remote>
