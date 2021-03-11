<script lang="ts">
  import { Button, Icon } from "../../../DesignSystem/Primitive";
  import {
    Copyable,
    Hoverable,
    Overlay,
  } from "../../../DesignSystem/Component";

  export let id: string;

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
  const caption = "Merge";
  const instructions = `git merge revisions/${id}`;
</script>

<style>
  .request-dropdown {
    margin-top: 3rem;
    right: 0;
    position: absolute;
    z-index: 1;
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
      To merge this merge request, run this in your terminal:
    </p>
    <Hoverable bind:hovering={hover}>
      <Copyable bind:this={copyable} showIcon={hover}>
        <p
          class="typo-text-small-mono"
          style="color: var(--color-foreground-level-6); overflow: scroll">
          {instructions}
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
    icon={Icon.Merge}
    on:click={toggleDropdown}
    dataCy="merge-request-modal-toggle">
    {caption}
  </Button>
</Overlay>
