<script>
  import { Button, Icon } from "../../DesignSystem/Primitive";
  import { Overlay } from "../../DesignSystem/Component";

  let expanded = false;

  const hide = () => (expanded = false);
  const toggleModal = () => {
    expanded = !expanded;
  };

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
      Anchoring is done through the radicle gnosis safe app.
    </p>

    <Button
      on:click={() => {
        // TODO(rudolfs): update the link.
        window.location.href = 'https://gnosis-safe.io/';
        toggleModal();
      }}
      style="margin-top: 1rem; width: 100%; display: block; text-align: center;">
      Anchor with Gnosis Safe
    </Button>
  </div>

  <Button variant="transparent" icon={Icon.Anchor} on:click={toggleModal}>
    Anchor a project
  </Button>
</Overlay>
