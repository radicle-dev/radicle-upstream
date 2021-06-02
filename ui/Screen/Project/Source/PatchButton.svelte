<script lang="typescript">
  import { Button, Icon } from "ui/DesignSystem/Primitive";
  import { Copyable, Overlay } from "ui/DesignSystem/Component";
  import * as patch from "ui/src/project/patch";

  export let expanded = false;
  const hide = () => (expanded = false);
  const toggleDropdown = () => {
    expanded = !expanded;
  };
  const caption = "New Patch";
</script>

<style>
  .request-dropdown {
    margin-top: 3rem;
    right: 0;
    position: absolute;
    border-radius: 1rem;
    background: var(--color-background);
    box-shadow: var(--color-shadows);
    padding: 1rem;
    width: 25rem;
  }

  p {
    color: var(--color-foreground-level-6);
    user-select: none;
    margin-bottom: 1rem;
  }

  p:last-child {
    margin-bottom: 0;
  }

  .instruction {
    color: var(--color-foreground-level-6);
    overflow-x: scroll;
    padding: 0.5rem 0.5rem 0.5rem 0.25rem;
  }
</style>

<Overlay {expanded} on:hide={hide} style="position: relative;">
  <div class="request-dropdown" hidden={!expanded}>
    <p>1. Make your changes and commit them locally.</p>
    <p>
      2. Create an annotated Git Tag that starts with
      <span class="typo-mono-bold">{patch.TAG_PREFIX}</span>
    </p>
    <Copyable style="margin-bottom: 1rem;">
      <pre
        class="typo-text-small-mono instruction">
        {`git tag --annotate ${patch.TAG_PREFIX}<name>`}
      </pre>
    </Copyable>
    <p>
      Be sure to replace
      <span class="typo-mono-bold">{`<name>`}</span>
      with the name of your Patch.
    </p>
    <p>
      3. In the tag message, add a title and description and save. Then, push
      your changes.
    </p>
    <Copyable style="margin-bottom: 1rem;">
      <pre
        class="typo-text-small-mono instruction">
        git push --tags rad
      </pre>
    </Copyable>
    <p>4. Your Patch is now public. 🎉</p>
  </div>

  <Button
    variant="transparent"
    icon={Icon.Revision}
    on:click={toggleDropdown}
    dataCy="patch-modal-toggle">
    {caption}
  </Button>
</Overlay>
