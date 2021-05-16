<script lang="typescript">
  import * as error from "../../src/error";
  import { openPath } from "../../src/ipc";
  import type { HorizontalItem } from "../../src/menu";
  import * as notification from "../../src/notification";
  import * as proxy from "../../src/proxy";
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
  import HorizontalMenu from "../../DesignSystem/Component/HorizontalMenu.svelte";
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

  export let activeTab:
    | "files"
    | "commits"
    | "commit"
    | "projects"
    | "following"
    | "funding";
  export let commitHash: string | null;

  const menuItems = (commitCount: number): HorizontalItem[] => {
    return [
      {
        icon: Icon.File,
        title: "Files",
        tab: "files",
      },
      {
        icon: Icon.Commit,
        title: "Commits",
        counter: commitCount,
        tab: "commits",
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
  const onMenuSelect = ({ detail: item }: { detail: HorizontalItem }) => {
    if (item.title === "Files" && activeTab === "files") {
      selectPath("");
    } else {
      if (item.tab !== null) {
        activeTab = item.tab;
      }
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

        <HorizontalMenu
          items={menuItems(history.stats.commits)}
          on:select={onMenuSelect}
          {activeTab} />
      </div>
    </div>
    <div slot="right">
      <CheckoutButton
        fork={!isContributor}
        on:checkout={ev => onCheckout(ev, project, selectedPeer)} />
    </div>
  </ActionBar>

  {#if activeTab === 'files'}
    <FilesTab />
  {:else if activeTab === 'commits'}
    <CommitsTab />
  {:else if activeTab === 'commit'}
    {#if commitHash}
      <CommitTab {commitHash} />
    {/if}
  {/if}
</Remote>
