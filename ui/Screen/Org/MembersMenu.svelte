<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script>
  import * as org from "ui/src/org";

  import { Button, Icon, Overlay } from "ui/DesignSystem";

  let expanded = false;

  const hide = () => (expanded = false);
  const toggleModal = () => {
    expanded = !expanded;
  };

  export let gnosisSafeAddress;
</script>

<style>
  .modal {
    margin-top: 3rem;
    right: 0;
    position: absolute;
    border-radius: 0.5rem;
    background: var(--color-background);
    border: 1px solid var(--color-foreground-level-3);
    box-shadow: var(--elevation-medium);
    padding: 1rem;
    width: 25rem;
  }

  p {
    color: var(--color-foreground-level-6);
    user-select: none;
  }
</style>

<Overlay {expanded} on:hide={hide} style="position: relative;">
  <div class="modal" hidden={!expanded}>
    <p style="margin-bottom: 0.5rem;">
      Org membership on Radicle is managed by Gnosis Safe, the most trusted
      platform for managing digital assets on Ethereum.
    </p>

    <Button
      on:click={() => {
        org.openOnGnosisSafe(gnosisSafeAddress, "settings");
        toggleModal();
      }}
      style="margin-top: 1rem; width: 100%; display: block; text-align: center;">
      Add a member on Gnosis Safe
    </Button>
  </div>

  <Button variant="transparent" icon={Icon.Plus} on:click={toggleModal}>
    Add a member
  </Button>
</Overlay>
