<script lang="typescript">
  import { get } from "svelte/store";
  import Router, { location } from "svelte-spa-router";

  import * as error from "../../src/error";
  import { openPath } from "../../src/ipc";
  import type { HorizontalItem } from "../../src/menu";
  import * as notification from "../../src/notification";
  import * as path from "../../src/path";
  import { checkout } from "../../src/project";
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
  import MergeRequestButton from "./Source/MergeRequestButton.svelte";

  import Code from "./Source/Code.svelte";
  import Commit from "./Source/Commit.svelte";
  import Commits from "./Source/Commits.svelte";
  import MergeRequests from "./Source/MergeRequests.svelte";
  import MergeRequest from "./Source/MergeRequest.svelte";

  export let project: Project;
  export let selectedPeer: User;
  export let isContributor: boolean;

  const routes = {
    "/projects/:urn/source/code": Code,
    "/projects/:urn/source/commit/:hash": Commit,
    "/projects/:urn/source/commits": Commits,
    "/projects/:urn/source/merge_requests": MergeRequests,
    "/projects/:urn/source/merge_request": MergeRequest,
  };
  let mergeRequestsTab = path.active(
    path.projectSourceMergeRequests(project.urn),
    get(location),
    true
  );

  const onCheckout = async (
    { detail: { checkoutPath } }: { detail: { checkoutPath: string } },
    project: Project,
    peer: User
  ) => {
    try {
      screen.lock();
      const path = await checkout(
        project.urn,
        checkoutPath,
        peer.identity.peerId
      );

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
      error.show({
        code: error.Code.ProjectCheckoutFailure,
        message: `Checkout failed: ${err.message}`,
        source: err,
      });
    } finally {
      screen.unlock();
    }
  };
  const onMenuSelect = ({ detail: item }: { detail: HorizontalItem }) => {
    if (
      item.title === "Files" &&
      path.active(path.projectSourceFiles(project.urn), get(location), true)
    ) {
      selectPath("");
    }
    mergeRequestsTab = item.title === "Merge Requests";
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

<Remote {store} let:data={{ menuItems, revisions, selectedRevision }}>
  <ActionBar>
    <div slot="left">
      <div style="display: flex">
        {#if !mergeRequestsTab}
          <div class="revision-selector-wrapper">
            <RevisionSelector
              loading={selectedRevision.request !== null}
              on:select={onSelectRevision}
              selected={selectedRevision.selected}
              defaultBranch={project.metadata.defaultBranch}
              {revisions} />
          </div>
        {/if}

        <HorizontalMenu items={menuItems} on:select={onMenuSelect} />
      </div>
    </div>
    <div slot="right">
      {#if mergeRequestsTab}
        <MergeRequestButton />
      {:else}
        <CheckoutButton
          fork={!isContributor}
          on:checkout={ev => onCheckout(ev, project, selectedPeer)} />
      {/if}
    </div>
  </ActionBar>

  <Router {routes} />
</Remote>
