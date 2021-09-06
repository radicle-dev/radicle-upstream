<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { User, Project } from "ui/src/project";
  import * as source from "ui/src/source";
  import * as project from "ui/src/project";
  import * as proxy from "ui/src/proxy";
  import * as modal from "ui/src/modal";
  import * as org from "ui/src/org";

  import { Button, Dropdown } from "ui/DesignSystem";

  import CommitTeaser from "ui/App/ProjectScreen/Source/SourceBrowser/CommitTeaser.svelte";
  import Modal from "ui/App/ModalLayout/Modal.svelte";
  import PeerSelector from "ui/App/ProjectScreen/PeerSelector.svelte";
  import RevisionSelector from "ui/App/ProjectScreen/Source/SourceBrowser/RevisionSelector.svelte";

  export let projects: org.ProjectOption[];
  export let orgAddress: string;
  export let ownerAddress: string;
  export let isMultiSig: boolean;

  // Project selector

  let selectedProjectUrn: string | undefined;

  interface ProjectData {
    project: Project;
    peers: User[];
    defaultBranchName: string;
    selectedPeer: User;
  }

  let projectData: ProjectData | undefined;

  $: if (selectedProjectUrn) {
    loadProjectData(selectedProjectUrn);
  }

  async function loadProjectData(projectUrn: string) {
    projectData = undefined;
    const prj = await proxy.client.project.get(projectUrn);
    const peers = await proxy.client.project.listPeers(projectUrn);
    const users = project.userList(peers);
    projectData = {
      project: prj,
      peers: users,
      defaultBranchName: prj.metadata.defaultBranch,
      selectedPeer: users[0],
    };
  }

  // Revision selector

  interface RevisionData {
    project: Project;
    peerId: string;
    revisions: Array<source.Tag | source.Branch>;
    selectedRevision: source.Tag | source.Branch;
    defaultBranchName: string;
  }

  let revisionData: RevisionData | undefined;

  $: if (projectData) {
    loadRevisionData(projectData);
  }

  async function loadRevisionData(projectData: ProjectData) {
    revisionData = undefined;

    const peerId = projectData.selectedPeer.peerId;
    const project = projectData.project;
    const { branches, tags } = await source.fetchRevisions(project.urn, peerId);
    const revisions = [...branches, ...tags];
    const defaultBranch = branches.find(
      (branch: source.Branch) =>
        branch.name === projectData.project.metadata.defaultBranch
    );
    revisionData = {
      project,
      peerId,
      revisions,
      defaultBranchName: project.metadata.defaultBranch,
      selectedRevision: defaultBranch || branches[0],
    };
  }

  $: if (revisionData) {
    loadCommitData(revisionData);
  }

  async function loadCommitData(revisionData: RevisionData) {
    const commits = await source.fetchCommits(
      revisionData.project.urn,
      revisionData.peerId,
      revisionData.selectedRevision
    );
    commit = commits.history[0];
  }

  async function createAnchor(): Promise<void> {
    if (!selectedProjectUrn || !commit) {
      return;
    }
    modal.hide();

    if (isMultiSig) {
      await org.anchorProjectWithGnosis(
        orgAddress,
        ownerAddress,
        selectedProjectUrn,
        commit.sha1
      );
    } else {
      await org.anchorProjectWithWallet(
        orgAddress,
        selectedProjectUrn,
        commit.sha1
      );
    }
  }

  let commit: source.CommitHeader | undefined;
</script>

<style>
  .revision-dropdown-container {
    display: flex;
    width: 100%;
    margin-bottom: 1.5rem;
    gap: 1rem;
  }

  .revision-dropdown-container > :global(*) {
    flex: 1;
  }
</style>

<Modal emoji="🏖️" title="Anchor a project">
  <div style="width: 100%; margin-bottom: 1.5rem;">
    <Dropdown
      bind:value={selectedProjectUrn}
      placeholder="Select a project"
      options={projects}
      menuStyle="width: 100%;" />
  </div>

  {#if projectData}
    <div class="revision-dropdown-container">
      <PeerSelector
        showProfile={false}
        rounded={true}
        peers={projectData.peers}
        on:select={event => {
          if (projectData) {
            projectData = {
              ...projectData,
              selectedPeer: event.detail,
            };
          }
        }}
        selected={projectData.selectedPeer} />
      {#if revisionData}
        <RevisionSelector
          loading={false}
          on:select={event => {
            if (revisionData) {
              revisionData = {
                ...revisionData,
                selectedRevision: event.detail,
              };
            }
          }}
          selected={revisionData.selectedRevision}
          defaultBranch={revisionData.defaultBranchName}
          revisions={revisionData.revisions} />
      {:else}
        <RevisionSelector
          loading={true}
          on:select={event => {
            if (revisionData) {
              revisionData = {
                ...revisionData,
                selectedRevision: event.detail,
              };
            }
          }}
          selected={{ type: source.RevisionType.Branch, name: "" }}
          defaultBranch=""
          revisions={[]} />
      {/if}
    </div>
  {/if}

  {#if commit}
    <CommitTeaser {commit} style="width: 100%; margin-bottom: 1.5rem" />
  {/if}

  <svelte:fragment slot="buttons">
    <Button variant="transparent" on:click={() => modal.hide()}>Cancel</Button>
    <Button disabled={!commit} on:click={createAnchor}
      >Confirm in your wallet</Button>
  </svelte:fragment>
</Modal>
