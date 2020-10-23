<script lang="ts">
  import { getContext } from "svelte";
  import { Variant as IllustrationVariant } from "../src/illustration";

  import {
    Illustration,
    List,
    TrackedRemoteListItem,
    UntrackedRemoteListItem,
  } from "../DesignSystem/Component";
  import { Button, Input } from "../DesignSystem/Primitive";

  const revisions = getContext("revisions");
  const metadata = getContext("metadata");

  const trackedRemotes = revisions.map((revision: any) => {
    return {
      handle: revision.identity.metadata.handle,
      peerID: revision.identity.peerId,
      maintainer: metadata.maintainers.includes(revision.identity.urn),
      avatarFallback: revision.identity.avatarFallback,
    };
  });

  // TODO(rudolfs): get these somewhere.
  const mockPendingRemotes = [
    {
      handle: null,
      peerID: "hwd1yreg4khbjfa4gsyrio3f7ehluwkdhyregs4k",
      maintainer: null,
    },
  ];
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

<div data-cy="remotes-modal" class="container">
  <Illustration
    style="margin-bottom: 1.5rem;"
    variant={IllustrationVariant.Computer} />
  <h1>Manage remotes</h1>
  <div class="input">
    <Input.Text
      dataCy="remote-input"
      placeholder="Enter a remote"
      style="width: 100%; margin-right: .5rem;" />
    <Button variant="secondary" disabled>Add remote</Button>
  </div>
  <List
    items={trackedRemotes}
    let:item={remote}
    style="width: 100%; margin: 1.5rem 0 0; padding: 0;">
    <TrackedRemoteListItem {remote} projectName={metadata.name} />
  </List>
  {#if mockPendingRemotes.length > 0}
    <div style="display: flex; width: 100%; margin-top: 1.5rem;">
      <p class="typo-text-bold">Still looking…</p>
      <p
        class="typo-text"
        style="margin-left: 0.5rem; color: var(--color-foreground-level-6);">
        These remotes haven’t been found yet.
      </p>
    </div>
    <List
      items={mockPendingRemotes}
      let:item={remote}
      style="width: 100%; margin: 1rem 0 0; padding: 0;">
      <UntrackedRemoteListItem {remote} projectName={metadata.name} />
    </List>
  {/if}
</div>
