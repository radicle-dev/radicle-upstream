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
  import { Variant as IllustrationVariant } from "../src/illustration";
  import {
    pendingPeers,
    peerSelection,
    project as store,
    trackPeer,
  } from "../src/project";

  let newRemote: PeerId;
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

  .input {
    display: flex;
    width: 100%;
    margin-top: 2rem;
  }
</style>

<Remote {store} let:data={project} context="project">
  <div data-cy="remotes-modal" class="container">
    <Illustration
      style="margin-bottom: 1.5rem;"
      variant={IllustrationVariant.Computer} />

    <h1>Manage remotes</h1>

    <div class="input">
      <Input.Text
        dataCy="remote-input"
        bind:value={newRemote}
        placeholder="Enter a remote"
        style="width: 100%; margin-right: .5rem;" />
      <Button
        variant="secondary"
        on:click={() => trackPeer(project.id, newRemote)}>
        Add remote
      </Button>
    </div>

    <Remote store={peerSelection} let:data>
      {#if data.peers.length > 0}
        <List
          items={data.peers}
          let:item={peer}
          styleHoverState={false}
          style="width: 100%; margin: 1.5rem 0 0; padding: 0;">
          <TrackedRemoteListItem {peer} projectName={project.metadata.name} />
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
          <UntrackedRemoteListItem {peer} projectName={project.metadata.name} />
        </List>
      {/if}
    </Remote>
  </div>
</Remote>
