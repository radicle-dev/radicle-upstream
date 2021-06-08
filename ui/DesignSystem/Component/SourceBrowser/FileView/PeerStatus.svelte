<script lang="typescript">
  import { Avatar, Button } from "ui/DesignSystem/Primitive";
  import { Copyable, Overlay } from "ui/DesignSystem/Component";

  export let ahead = 3;
  export let behind = 5;
  export let branchName = "main";

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
    instructions = `git pull rad main:main`;
  }

</script>

<style>
  .btn-content {
    display: flex;
    align-items: center;
    white-space: nowrap;
    gap: 0.5rem;
  }

  .request-dropdown {
    margin-top: 3rem;
    right: 0;
    position: absolute;
    z-index: 1;
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
  }

  .instructions {
    color: var(--color-foreground-level-6);
    overflow-x: scroll;
    padding: 0.5rem 0.5rem 0.5rem 0.25rem;
    flex: 1;
  }

</style>

<Overlay {expanded} on:hide={hide} style="position: relative;">
  <div class="request-dropdown" hidden={!expanded}>
    <p style="margin-bottom: 0.5rem;">
      To merge in the maintainers' main branch, run:
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

  <Button on:click={toggleDropdown} dataCy="peer-status" variant="outline">
    <div class="btn-content">
      <p class="typo-text-bold">{ahead} ahead, {behind} behind</p>
      <p class="typo-text">of</p>
      <Avatar size="small" />
      <p class="typo-text-bold">{branchName}</p>
    </div>
  </Button>
</Overlay>
