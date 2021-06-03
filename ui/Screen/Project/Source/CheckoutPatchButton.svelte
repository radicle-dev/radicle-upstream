<script lang="typescript">
  import { Button, Icon } from "../../../DesignSystem/Primitive";
  import { Copyable, Overlay } from "../../../DesignSystem/Component";
  import * as Patch from "../../../src/project/patch";

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
    const peerLabel = patch.identity
      ? patch.identity.metadata.handle
      : patch.peerId;
    const localRef = `tags/${Patch.TAG_PREFIX}${peerLabel}/${patch.id}`;
    let remoteRef = `tags/${Patch.TAG_PREFIX}${patch.id}`;
    if (patch.peerId !== myPeerId) {
      remoteRef = `remotes/${patch.peerId}/${remoteRef}`;
    }
    instructions = `git fetch rad ${remoteRef}:${localRef}\ngit checkout ${localRef}`;
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
    white-space: pre;
  }
</style>

<Overlay {expanded} on:hide={hide} style="position: relative;">
  <div class="request-dropdown" hidden={!expanded}>
    <p style="margin-bottom: 0.5rem;">
      To check out this Patch locally, run this in your terminal:
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
    variant="transparent"
    icon={Icon.ArrowBoxUpRight}
    on:click={toggleDropdown}
    dataCy="checkout-patch-modal-toggle">
    Checkout
  </Button>
</Overlay>
