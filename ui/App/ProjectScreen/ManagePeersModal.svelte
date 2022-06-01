<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { TextInputValidationState } from "design-system/TextInput";
  import type { User } from "ui/src/project";

  import * as svelteStore from "svelte/store";

  import * as project from "ui/src/project";
  import * as projectScreen from "ui/src/screen/project";
  import * as remote from "ui/src/remote";

  import Button from "design-system/Button.svelte";
  import List from "design-system/List.svelte";
  import TextInput from "design-system/TextInput.svelte";

  import Modal from "ui/App/ModalLayout/Modal.svelte";
  import Peer from "./ManagePeers/Peer.svelte";
  import PeerTrackRequest from "./ManagePeers/PeerTrackRequest.svelte";

  const projectScreenStore = projectScreen.store;

  let newPeer: string;
  let peerValidation: TextInputValidationState = { type: "unvalidated" };

  // Don't show our own peer in the list unless we have published something.
  function filteredPeers(peers: User[]): User[] {
    return peers.filter(peer => {
      return !(
        peer.type === project.PeerType.Local &&
        peer.role === project.PeerRole.Tracker
      );
    });
  }

  const pendingPeers = svelteStore.derived(projectScreenStore, remoteData => {
    if (remoteData.status === remote.Status.Success) {
      return remoteData.data.peers.filter(
        peer =>
          peer.status.type === project.PeerReplicationStatusType.NotReplicated
      );
    } else {
      return [];
    }
  });

  function isPeerAlreadyTracked(peer: string): boolean {
    if ($projectScreenStore.status === remote.Status.Success) {
      return !$projectScreenStore.data.peers
        .map(peer => {
          return peer.peerId;
        })
        .includes(peer);
    }

    return false;
  }

  function isPeerValid(peerId: string): TextInputValidationState {
    if (!peerId.match(projectScreen.VALID_PEER_MATCH)) {
      return {
        type: "invalid",
        message: "This is not a valid remote",
      };
    }

    if (!isPeerAlreadyTracked(peerId)) {
      return {
        type: "invalid",
        message: "This remote is already being tracked",
      };
    }

    return { type: "valid" };
  }

  $: if (newPeer === "") {
    peerValidation = { type: "unvalidated" };
  }
</script>

<style>
  .peer-entry-form {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: 100%;
  }

  .peer-entry-field {
    display: flex;
    justify-content: flex-start;
    margin-top: 2rem;
    width: 100%;
  }
</style>

{#if $projectScreenStore.status === remote.Status.Success}
  <Modal dataCy="remotes-modal" emoji="💻" title="Edit remotes">
    <svelte:fragment slot="description">
      Add a user’s Peer ID to collaborate with them on this project.
    </svelte:fragment>

    <form class="peer-entry-form" on:submit|preventDefault>
      <div class="peer-entry-field">
        <TextInput
          dataCy="peer-input"
          bind:value={newPeer}
          placeholder="Enter a Peer ID here"
          validationState={peerValidation}
          style="margin-right: .5rem;" />
        <Button
          dataCy="track-button"
          style="display: flex; align-self: flex-start;"
          disabled={!newPeer}
          on:click={() => {
            if ($projectScreenStore.status === remote.Status.Success) {
              peerValidation = isPeerValid(newPeer);

              if (peerValidation.type === "valid") {
                projectScreen.addPeer(
                  $projectScreenStore.data.project.urn,
                  newPeer
                );

                newPeer = "";
              }
            }
          }}>
          Add
        </Button>
      </div>
    </form>

    <List
      dataCy="tracked-peers"
      key="peerId"
      items={filteredPeers($projectScreenStore.data.peerSelection)}
      let:item={peer}
      styleHoverState={false}
      style="width: 100%; margin: 1.5rem 0 0; padding: 0;">
      <Peer
        {peer}
        on:untrack={event => {
          projectScreen.removePeer(
            event.detail.projectUrn,
            event.detail.peerId
          );

          peerValidation = { type: "unvalidated" };
        }}
        projectUrn={$projectScreenStore.data.project.urn} />
    </List>

    {#if $pendingPeers.length > 0}
      <div style="display: flex; width: 100%; margin-top: 1.5rem;">
        <p class="typo-text-bold">Still looking…</p>
        <p
          class="typo-text"
          style="margin-left: 0.5rem; color: var(--color-foreground-level-6);">
          These remotes haven’t been found yet.
        </p>
      </div>
    {/if}

    <List
      dataCy="pending-peers"
      key="peerId"
      items={$pendingPeers}
      let:item={peer}
      styleHoverState={false}
      style="width: 100%; margin: 1rem 0 0; padding: 0;">
      <PeerTrackRequest
        {peer}
        on:cancel={event => {
          projectScreen.removePeer(
            event.detail.projectUrn,
            event.detail.peerId
          );

          peerValidation = { type: "unvalidated" };
        }}
        projectUrn={$projectScreenStore.data.project.urn} />
    </List>
  </Modal>
{/if}
