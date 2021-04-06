<script lang="typescript">
  import { Button, Icon } from "../../../DesignSystem/Primitive";
  import { Copyable, Overlay } from "../../../DesignSystem/Component";
  import * as mergeRequest from "../../../src/project/mergeRequest";

  export let expanded = false;
  const hide = () => (expanded = false);
  const toggleDropdown = () => {
    expanded = !expanded;
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
</style>

<Overlay {expanded} on:hide={hide} style="position: relative;">
  <div class="request-dropdown" hidden={!expanded}>
    <p>1. Make your changes.</p>
    <p>
      2. Create an annotated Git Tag that starts with
      <span class="typo-mono-bold">${mergeRequest.tagPrefix}</span>
    </p>
    <Copyable style="margin-bottom: 1rem;">
      <pre
        class="typo-text-small-mono"
        style="color: var(--color-foreground-level-6); overflow-x: scroll; padding: .5rem .5rem .5rem .25rem">
        {`git tag --annotate ${mergeRequest.tagPrefix}/<name>`}
      </pre>
    </Copyable>
    <p>
      Be sure to replace
      <span class="typo-mono-bold">{`<name>`}</span>
      with the name of your merge request.
    </p>
    <p>
      3. In the tag message, add a title and description and save. Then, push
      your changes.
    </p>
    <Copyable style="margin-bottom: 1rem;">
      <pre
        class="typo-text-small-mono"
        style="color: var(--color-foreground-level-6); overflow-x: scroll; padding: .5rem .5rem .5rem .25rem">
        git push --tags rad
      </pre>
    </Copyable>
    <p>4. Your merge request is now public. 🎉</p>
  </div>

  <Button
    variant="transparent"
    icon={Icon.Revision}
    on:click={toggleDropdown}
    dataCy="merge-request-modal-toggle">
    {caption}
  </Button>
</Overlay>
