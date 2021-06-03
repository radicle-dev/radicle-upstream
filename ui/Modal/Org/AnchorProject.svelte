<script lang="typescript">
  import type { User } from "ui/src/project";
  import type * as urn from "ui/src/urn";
  import type * as source from "ui/src/source";
  import type { Writable } from "svelte/store";

  import * as modal from "ui/src/modal";
  import * as org from "ui/src/org";
  import * as projectScreen from "ui/src/screen/project";
  import * as projectSourceScreen from "ui/src/screen/project/source";
  import * as remote from "ui/src/remote";

  import { Button, Emoji } from "ui/DesignSystem/Primitive";
  import {
    Dropdown,
    Modal,
    Remote,
    PeerSelector,
    RevisionSelector,
  } from "ui/DesignSystem/Component";
  import CommitTeaser from "ui/DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";

  const projectScreenStore = projectScreen.store;
  const projectSourceScreenStore = projectSourceScreen.store;

  export let projects: org.ProjectOption[];
  let projectUrn: urn.Urn | undefined = undefined;
  const commitTeaser = undefined;

  $: if (projectUrn) {
    projectScreen.fetch(projectUrn);
  }

  $: if ($projectScreenStore.status === remote.Status.Success) {
    projectSourceScreen.fetch(
      $projectScreenStore.data.project,
      $projectScreenStore.data.selectedPeer
    );
  }

  let code: Writable<projectSourceScreen.Code>;
  $: if ($projectSourceScreenStore.status === remote.Status.Success) {
    code = $projectSourceScreenStore.data.code;
  }

  const onSelectPeer = ({ detail: peer }: { detail: User }) => {
    projectScreen.selectPeer(peer);
  };

  const onSelectRevision = ({
    detail: revision,
  }: {
    detail: source.Branch | source.Tag;
  }) => {
    projectSourceScreen.selectRevision(revision);
  };
</script>

<style>
  .actions {
    display: flex;
    width: 100%;
    gap: 1.5rem;
    justify-content: flex-end;
  }
</style>

<Modal>
  <Emoji emoji={"🏖️"} size="huge" style="margin-bottom: 1.5rem;" />
  <h1 style="margin-bottom: 1.5rem;">Anchor a project</h1>

  <div style="width: 100%; margin-bottom: 1.5rem;">
    <Dropdown
      bind:value={projectUrn}
      placeholder="Select a project"
      options={projects}
      menuStyle="width: 100%;" />
  </div>
  <Remote
    store={projectScreen.store}
    let:data={{
      peerSelection,
      project,
      selectedPeer,
    }}>
    <div style="display: flex; width: 100%; margin-bottom: 1.5rem;">
      <div style="width: 100%; margin-right: 1rem;">
        <PeerSelector
          showProfile={false}
          peers={peerSelection}
          on:select={onSelectPeer}
          selected={selectedPeer} />
      </div>
      <div style="width: 100%;">
        {#if $projectSourceScreenStore.status === remote.Status.Success}
          <RevisionSelector
            loading={$projectSourceScreenStore.data.selectedRevision.request !==
              null}
            on:select={onSelectRevision}
            selected={$projectSourceScreenStore.data.selectedRevision.selected}
            defaultBranch={project.metadata.defaultBranch}
            revisions={$projectSourceScreenStore.data.revisions} />
        {/if}
      </div>
    </div>
    {#if $projectSourceScreenStore.status === remote.Status.Success}
      {#if $projectSourceScreenStore.data.code}
        <CommitTeaser
          style="width: 100%; margin-bottom: 1.5rem;"
          commit={$code.lastCommit} />
      {/if}
    {/if}
  </Remote>
  <div class="actions">
    <Button variant="transparent" on:click={() => modal.hide()}>Cancel</Button>
    <Button
      disabled={!commitTeaser}
      on:click={() => {
        org.anchorProject();
      }}>Confirm in your wallet</Button>
  </div>
</Modal>
