<script lang="typescript">
  import Router from "svelte-spa-router";

  import * as error from "../../src/error";
  import { openPath } from "../../src/ipc";
  import * as notification from "../../src/notification";
  import { checkout } from "../../src/project";
  import type { Project, User } from "../../src/project";
  import {
    fetch,
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

  import Code from "./Source/Code.svelte";
  import Commit from "./Source/Commit.svelte";
  import Commits from "./Source/Commits.svelte";

  export let project: Project;
  export let selectedPeer: User;

  const routes = {
    "/projects/:urn/source/code": Code,
    "/projects/:urn/source/commit/:hash": Commit,
    "/projects/:urn/source/commits": Commits,
  };

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

      notification.info(
        `${project.metadata.name} checked out to ${path}`,
        true,
        "Open folder",
        () => {
          openPath(path);
        }
      );
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
        <div class="revision-selector-wrapper">
          <RevisionSelector
            loading={selectedRevision.request !== null}
            on:select={onSelectRevision}
            selected={selectedRevision.selected}
            {revisions} />
        </div>

        <HorizontalMenu items={menuItems} />
      </div>
    </div>
    <div slot="right">
      <CheckoutButton
        on:checkout={ev => onCheckout(ev, project, selectedPeer)} />
    </div>
  </ActionBar>

  <Router {routes} />
</Remote>
