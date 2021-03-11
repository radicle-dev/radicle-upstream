<script>
  import { Button, Icon } from "../../../DesignSystem/Primitive";
  import {
    Copyable,
    Hoverable,
    Overlay,
  } from "../../../DesignSystem/Component";

  let expanded = false;
  let hover = false;
  let copyable;
  const hide = () => (expanded = false);
  const toggleDropdown = () => {
    expanded = !expanded;
  };
  const copy = () => {
    copyable.copy();
    toggleDropdown();
  };
  const caption = "New Merge Request";
</script>

<style>
  .request-dropdown {
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
  <div class="request-dropdown" hidden={!expanded}>
    <p style="margin-bottom: 0.5rem;">
      To create a new merge request, run this in your terminal:
    </p>
    <Hoverable bind:hovering={hover}>
      <Copyable bind:this={copyable} showIcon={hover}>
        <p
          class="typo-text-small-mono"
          style="color: var(--color-foreground-level-6)">
          git tag --annotate merge-request/name-your-tag
          <br />
          git push --tags rad
        </p>
      </Copyable>
    </Hoverable>
    <Button
      variant="transparent"
      style="display: block; margin: auto;"
      on:click={copy}>
      Copy
    </Button>
  </div>

  <Button
    variant="transparent"
    icon={Icon.Revision}
    on:click={toggleDropdown}
    dataCy="merge-request-modal-toggle">
    {caption}
  </Button>
</Overlay>
