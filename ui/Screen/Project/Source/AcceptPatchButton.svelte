<script lang="typescript">
  import { Button, Icon } from "ui/DesignSystem/Primitive";
  import { Copyable, Overlay } from "ui/DesignSystem/Component";
  import * as Patch from "ui/src/project/patch";

  export let patch: Patch.Patch;
  export let myPeerId: string;

  let expanded = false;
  let copyable: Copyable;
  const hide = () => (expanded = false);
  const toggleDropdown = () => {
    expanded = !expanded;
  };
  const copy = () => {
    copyable.copy();
    toggleDropdown();
  };

  let instructions: string;
  $: {
    let remoteRef = `tags/${Patch.TAG_PREFIX}${patch.id}`;
    if (patch.peerId !== myPeerId) {
      remoteRef = `remotes/${patch.peerId}/${remoteRef}`;
    }
    instructions = `git pull rad ${remoteRef}`;
  }
</script>

<style>
  .request-dropdown {
    margin-top: 3rem;
    right: 0;
    position: absolute;
    z-index: 1;
    border-radius: 1rem;
    background: var(--color-background);
    box-shadow: var(--color-shadows);
    padding: 1rem;
    width: 25rem;
  }

  p {
    color: var(--color-foreground-level-6);
    user-select: none;
  }

  .instructions {
    color: var(--color-foreground-level-6);
    overflow-x: scroll;
    padding: 0.5rem 0.5rem 0.5rem 0.25rem;
  }
</style>

<Overlay {expanded} on:hide={hide} style="position: relative;">
  <div class="request-dropdown" hidden={!expanded}>
    <p style="margin-bottom: 0.5rem;">
      To merge this Patch, run this in your terminal:
    </p>
    <Copyable bind:this={copyable} showIcon={false}>
      <p class="typo-text-small-mono instructions">{instructions}</p>
    </Copyable>
    <Button
      style="display: block; margin: 1rem auto 0; width: 100%;"
      on:click={copy}>
      Copy
    </Button>
  </div>

  <Button
    icon={Icon.Merge}
    on:click={toggleDropdown}
    dataCy="merge-patch-modal-toggle">
    Merge
  </Button>
</Overlay>
