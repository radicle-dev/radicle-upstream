<script lang="typescript">
  import { onDestroy } from "svelte";
  import Router, { location } from "svelte-spa-router";
  import * as router from "svelte-spa-router/wrap";

  import * as error from "../../src/error";
  import { openPath } from "../../src/ipc";
  import type { HorizontalItem } from "../../src/menu";
  import * as notification from "../../src/notification";
  import * as path from "../../src/path";
  import * as proxy from "../../src/proxy";
  import type { Project, User } from "../../src/project";
  import {
    fetch,
    watchPatchUpdates,
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
  import PatchButton from "./Source/PatchButton.svelte";

  import Code from "./Source/Code.svelte";
  import Commit from "./Source/Commit.svelte";
  import Commits from "./Source/Commits.svelte";
  import Patches from "./Source/Patches.svelte";
  import Patch from "./Source/Patch.svelte";

  export let project: Project;
  export let selectedPeer: User;
  export let isContributor: boolean;

  const routes = {
    "/projects/:urn/source/code": Code,
    "/projects/:urn/source/commit/:hash": Commit,
    "/projects/:urn/source/commits": Commits,
    "/projects/:urn/source/patches": Patches,
    "/projects/:urn/source/patch/:peerId/:id": router.wrap({
      component: Patch,
      props: {
        project,
      },
    }),
  };
  $: patchesTabSelected =
    $location.startsWith(path.projectSourcePatches(project.urn)) ||
    $location.startsWith(`/projects/${project.urn}/source/patch`);

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
    if (
      item.title === "Files" &&
      $location.startsWith(path.projectSourceFiles(project.urn))
    ) {
      selectPath("");
    }
  };
  const onSelectRevision = ({ detail: revision }: { detail: Branch | Tag }) => {
    selectRevision(revision);
  };

  const unwatchPatchUpdates = watchPatchUpdates();
  onDestroy(unwatchPatchUpdates);
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
        {#if !patchesTabSelected}
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
      {#if patchesTabSelected}
        <PatchButton />
      {:else}
        <CheckoutButton
          fork={!isContributor}
          on:checkout={ev => onCheckout(ev, project, selectedPeer)} />
      {/if}
    </div>
  </ActionBar>

  <Router {routes} />
</Remote>
