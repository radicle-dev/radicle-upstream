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
  import CodeTab from "ui/Screen/Project/Source/Code.svelte";

  export let project: Project;
  export let selectedPeer: User;
  export let isContributor: boolean;

  export let activeTab: typeof SvelteComponent;
  export let commitHash: string;

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
    if (item.title === "Files" && activeTab === CodeTab) {
      selectPath("");
    } else {
      activeTab = item.tab.component;
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
            defaultBranch={project.metadata.defaultBranch}
            {revisions} />
        </div>

        <HorizontalMenu items={menuItems} on:select={onMenuSelect} />
      </div>
    </div>
    <div slot="right">
      <CheckoutButton
        fork={!isContributor}
        on:checkout={ev => onCheckout(ev, project, selectedPeer)} />
    </div>
  </ActionBar>

  <svelte:component this={activeTab} {commitHash} />
</Remote>
