<script lang="ts">
  import { List, RemoteListItem } from "../DesignSystem/Component";
  import { Emoji, Button, Input } from "../DesignSystem/Primitive";

  const remotes = [
    {
      handle: "cloudhead",
      peerID: "hwd1yreg4khbjfa4gsyrio3f7ehluwkdhyregs4k",
      maintainer: true,
      synced: true,
    },
    {
      handle: "juliendonck",
      peerID: "hwd1yreg4khbjfa4gsyrio3f7ehluwkdhyregs4k",
      maintainer: false,
      synced: true,
    },
    {
      handle: null,
      peerID: "hwd1yreg4khbjfa4gsyrio3f7ehluwkdhyregs4k",
      maintainer: null,
      synced: false,
    },
  ];

  const syncedRemotes = remotes.filter(remote => remote.synced === true);
  const unsyncedRemotes = remotes.filter(remote => remote.synced === false);
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
  <Emoji emoji={'💻'} size="huge" style="margin-bottom: 1.5rem;" />
  <h1>Manage remotes</h1>
  <div class="input">
    <Input.Text
      dataCy="remote-input"
      placeholder="Enter a remote"
      style="width: 100%; margin-right: .5rem;" />
    <Button variant="secondary" disabled>Add remote</Button>
  </div>
  <List
    items={syncedRemotes}
    let:item={remote}
    style="width: 100%; margin: 1.5rem 0 0; padding: 0;">
    <RemoteListItem {remote} />
  </List>
  {#if unsyncedRemotes.length > 0}
    <div style="display: flex; width: 100%; margin-top: 1.5rem;">
      <p class="typo-text-bold">Still looking…</p>
      <p
        class="typo-text"
        style="margin-left: 0.5rem; color: var(--color-foreground-level-6);">
        These remotes haven’t been found yet.
      </p>
    </div>
    <List
      items={unsyncedRemotes}
      let:item={remote}
      style="width: 100%; margin: 1rem 0 0; padding: 0;">
      <RemoteListItem {remote} />
    </List>
  {/if}
</div>
