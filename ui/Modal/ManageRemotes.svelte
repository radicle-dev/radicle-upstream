<script lang="ts">
  import { Button, Input } from "../DesignSystem/Primitive";
  import {
    Illustration,
    List,
    Remote,
    TrackedRemoteListItem,
    UntrackedRemoteListItem,
  } from "../DesignSystem/Component";

  import type { PeerId } from "../src/identity";
  import type { Urn } from "../src/urn";

  import { Variant as IllustrationVariant } from "../src/illustration";
  import {
    addRemote,
    pendingPeers,
    peerSelection,
    peerValidation,
    project as store,
    removeRemote,
  } from "../src/project";

  let newRemote: PeerId;

  $: if (newRemote === "") {
    peerValidation.reset();
  }

  const submitRemote = async (projectUrn: Urn) => {
    if (await addRemote(projectUrn, newRemote)) {
      newRemote = "";
    }
  };

  const cancelFollowRequest = (projectUrn: Urn, peerId: PeerId) => {
    removeRemote(projectUrn, peerId);
  };

  const unfollowRemote = (projectUrn: Urn, peerId: PeerId) => {
    removeRemote(projectUrn, peerId);
  };
</script>

<style>
  .container {
    width: 38.5rem;
    background: var(--color-background);
    border-radius: 0.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
  }

  .container:focus {
    outline: none;
  }

  .remote-entry-form {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: 100%;
  }

  .remote-entry-field {
    display: flex;
    justify-content: flex-start;
    margin-top: 2rem;
    width: 100%;
  }
</style>

<Remote {store} let:data={project} context="project">
  <div data-cy="remotes-modal" class="container">
    <Illustration
      style="margin-bottom: 1.5rem;"
      variant={IllustrationVariant.Computer} />

    <h1>Manage remotes</h1>

    <form class="remote-entry-form" on:submit|preventDefault>
      <div class="remote-entry-field">
        <Input.Text
          hint="v"
          dataCy="remote-input"
          bind:value={newRemote}
          placeholder="Paste a remote address here"
          validation={$peerValidation}
          style="width: 100%; margin-right: .5rem;" />
        <Button
          style="display: flex; align-self: flex-start;"
          variant="secondary"
          disabled={!newRemote}
          on:click={() => submitRemote(project.id)}>
          Add remote
        </Button>
      </div>
    </form>

    <Remote store={peerSelection} let:data>
      {#if data.peers.length > 0}
        <List
          items={data.peers}
          let:item={peer}
          styleHoverState={false}
          style="width: 100%; margin: 1.5rem 0 0; padding: 0;">
          <TrackedRemoteListItem
            {peer}
            on:unfollow={event => {
              unfollowRemote(event.detail.projectUrn, event.detail.peerId);
            }}
            projectName={project.metadata.name}
            projectUrn={project.id} />
        </List>
      {/if}
    </Remote>

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
        <List
          items={data.peers}
          let:item={peer}
          styleHoverState={false}
          style="width: 100%; margin: 1rem 0 0; padding: 0;">
          <UntrackedRemoteListItem
            {peer}
            on:cancel={event => {
              cancelFollowRequest(event.detail.projectUrn, event.detail.peerId);
            }}
            projectName={project.metadata.name}
            projectUrn={project.id} />
        </List>
      {/if}
    </Remote>
  </div>
</Remote>
