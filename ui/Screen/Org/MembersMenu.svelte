<script>
  import * as ipc from "ui/src/ipc";

  import { Button, Icon } from "ui/DesignSystem/Primitive";
  import { Overlay } from "ui/DesignSystem/Component";

  let expanded = false;

  const hide = () => (expanded = false);
  const toggleModal = () => {
    expanded = !expanded;
  };

  export let gnosisSafeAddress;

  // TODO(rudolfs): make the link go to
  // `https://gnosis-safe.io/app/#/safes/${gnosisSafeAddress}/settings` for
  // mainnet
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
      Organization membership on Radicle is managed by Gnosis Safe, the most
      trusted platform for managing digital assets and organizations on
      Ethereum.
    </p>

    <Button
      on:click={() => {
        ipc.openUrl(
          `https://rinkeby.gnosis-safe.io/app/#/safes/${gnosisSafeAddress}/settings`
        );
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
