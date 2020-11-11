<script lang="typescript">
  import Router, { push } from "svelte-spa-router";

  import { openPath } from "../../src/ipc";
  import * as menu from "../../src/menu";
  import * as notification from "../../src/notification";
  import * as path from "../../src/path";
  import { checkout } from "../../src/project";
  import type { Project, User } from "../../src/project";
  import {
    commits as commitsStore,
    revisionSelection,
    selectedPeer,
    selectRevision,
    selectedRevision,
  } from "../../src/screen/project";
  import type { Revision } from "../../src/source";
  import * as screen from "../../src/screen";

  import IconCommit from "../../DesignSystem/Primitive/Icon/Commit.svelte";
  import IconHouse from "../../DesignSystem/Primitive/Icon/House.svelte";

  import ActionBar from "../../DesignSystem/Component/ActionBar.svelte";
  import HorizontalMenu from "../../DesignSystem/Component/HorizontalMenu.svelte";
  import Remote from "../../DesignSystem/Component/Remote.svelte";
  import RevisionSelector from "../../DesignSystem/Component/SourceBrowser/RevisionSelector.svelte";

  import CheckoutButton from "./Source/CheckoutButton.svelte";

  import Code from "./Source/Code.svelte";
  import Commit from "./Source/Commit.svelte";
  import Commits from "./Source/Commits.svelte";

  export let project: Project;

  const routes = {
    "/projects/:urn/source/code": Code,
    "/projects/:urn/source/commit/:hash": Commit,
    "/projects/:urn/source/commits": Commits,
  };

  const onCheckout = async (
    { detail: { checkoutPath } }: { detail: { checkoutPath: string } },
    project: Project,
    peer: User | null
  ) => {
    if (peer === null) {
      notification.error(`Can't checkout without a peer selected`);
      return;
    }

    try {
      screen.lock();
      const path = await checkout(
        project.urn,
        checkoutPath,
        peer.identity.peerId
      );

      notification.info(
        `${project.metadata.name} checked out to ${path}`,
        true,
        "Open folder",
        () => {
          openPath(path);
        }
      );
    } catch (error) {
      notification.error(`Checkout failed: ${error.message}`, true);
    } finally {
      screen.unlock();
    }
  };
  const onSelectRevision = ({ detail: revision }: { detail: Revision }) => {
    selectRevision(revision);
  };
  const topbarMenuItems = (
    project: Project,
    commitCounter?: number
  ): menu.HorizontalItem[] => {
    const items = [
      {
        icon: IconHouse,
        title: "Source",
        href: path.projectSourceCode(project.urn),
        looseActiveStateMatching: true,
      },
      {
        icon: IconCommit,
        title: "Commits",
        counter: commitCounter,
        href: path.projectSourceCommits(project.urn),
        looseActiveStateMatching: true,
      },
    ];
    return items;
  };

  push(path.projectSourceCode(project.urn));
</script>

<style>
  .revision-selector-wrapper {
    width: 18rem;
    position: relative;
    margin-right: 2rem;
  }
</style>

<ActionBar>
  <div slot="left">
    <!-- FIXME(xla): These elements belong in Source.svelte -->
    <div style="display: flex">
      <Remote store={revisionSelection} let:data={revisions}>
        <div class="revision-selector-wrapper">
          <RevisionSelector
            on:select={onSelectRevision}
            selected={$selectedRevision || revisions.default}
            {revisions} />
        </div>
      </Remote>
      <Remote store={commitsStore} let:data={commits}>
        <HorizontalMenu
          items={topbarMenuItems(project, commits.stats.commits)} />
        <div slot="loading">
          <HorizontalMenu
            items={topbarMenuItems(project)}
            style="display: inline" />
        </div>
      </Remote>
    </div>
  </div>
  <div slot="right">
    <CheckoutButton
      on:checkout={ev => onCheckout(ev, project, $selectedPeer)} />
  </div>
</ActionBar>

<Router {routes} />
