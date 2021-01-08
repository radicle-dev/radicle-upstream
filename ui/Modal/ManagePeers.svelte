<script lang="typescript">
  import type { PeerId } from "../src/identity";
  import { PeerType, Role } from "../src/project";
  import type { User } from "../src/project";
  import {
    addPeer,
    pendingPeers,
    peerValidation,
    removePeer,
    store,
  } from "../src/screen/project";
  import type { Urn } from "../src/urn";

  import { Button, Emoji, Input } from "../DesignSystem/Primitive";
  import { List, Remote } from "../DesignSystem/Component";

  import Peer from "./ManagePeers/Peer.svelte";
  import PeerFollowRequest from "./ManagePeers/PeerFollowRequest.svelte";

  let newPeer: PeerId;

  $: if (newPeer === "") {
    peerValidation.reset();
  }

  const submitPeer = async (projectUrn: Urn) => {
    if (await addPeer(projectUrn, newPeer)) {
      newPeer = "";
    }
  };

  const cancelFollowRequest = (projectUrn: Urn, peerId: PeerId) => {
    removePeer(projectUrn, peerId);
    peerValidation.reset();
  };

  const unfollowPeer = (projectUrn: Urn, peerId: PeerId) => {
    removePeer(projectUrn, peerId);
    peerValidation.reset();
  };

  // Don't show our own peer in the list unless we have published something.
  const filteredPeers = (peers: [User]) => {
    return peers.filter(peer => {
      return !(peer.type === PeerType.Local && peer.role === Role.Tracker);
    });
  };
</script>

<style>
  .container {
    width: 38.5rem;
    background: var(--color-background);
    border-radius: 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
  }

  .container:focus {
    outline: none;
  }

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

<Remote {store} let:data={{ peerSelection, project }}>
  <div data-cy="remotes-modal" class="container">
    <Emoji emoji={'💻'} size="huge" style="margin-bottom: 1.5rem;" />

    <h1>Manage remotes</h1>
    <p style="margin-top: 0.5rem; color: var(--color-foreground-level-6);">
      Add a user's Device ID to collaborate with them on this project.
    </p>

    <form class="peer-entry-form" on:submit|preventDefault>
      <div class="peer-entry-field">
        <Input.Text
          dataCy="peer-input"
          bind:value={newPeer}
          placeholder="Enter a Device ID here"
          validation={$peerValidation}
          style="width: 100%; margin-right: .5rem;" />
        <Button
          dataCy="follow-button"
          style="display: flex; align-self: flex-start;"
          variant="secondary"
          disabled={!newPeer}
          on:click={() => submitPeer(project.urn)}>
          Add
        </Button>
      </div>
    </form>

    <List
      dataCy="followed-peers"
      key="peerId"
      items={filteredPeers(peerSelection)}
      let:item={peer}
      styleHoverState={false}
      style="width: 100%; margin: 1.5rem 0 0; padding: 0;">
      <Peer
        {peer}
        on:unfollow={event => {
          unfollowPeer(event.detail.projectUrn, event.detail.peerId);
        }}
        projectUrn={project.urn} />
    </List>

    <Remote store={pendingPeers} let:data>
      {#if data.peers.length > 0}
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
        items={data.peers}
        let:item={peer}
        styleHoverState={false}
        style="width: 100%; margin: 1rem 0 0; padding: 0;">
        <PeerFollowRequest
          {peer}
          on:cancel={event => {
            cancelFollowRequest(event.detail.projectUrn, event.detail.peerId);
          }}
          projectUrn={project.urn} />
      </List>
    </Remote>
  </div>
</Remote>
