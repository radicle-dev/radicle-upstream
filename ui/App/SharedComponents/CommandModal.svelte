<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Overlay from "design-system/Overlay.svelte";
  import Copyable from "ui/App/SharedComponents/Copyable.svelte";

  export let command: string;
  export let description: string;
  export let dataCy: string | undefined = undefined;

  let expanded = false;

  const toggleDropdown = (): void => {
    expanded = !expanded;
  };
</script>

<style>
  .container {
    position: relative;
    margin-left: auto;
    align-self: center;
    text-align: left;
  }

  .dropdown {
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
    margin-bottom: 0.5rem;
  }

  .command {
    color: var(--color-foreground-level-6);
    overflow-x: scroll;
    padding: 0.5rem 0.5rem 0.5rem 0.25rem;
    white-space: pre;
  }
</style>

<div class="container" data-cy={dataCy}>
  <Overlay
    {expanded}
    on:hide={() => {
      expanded = false;
    }}>
    <div class="dropdown" hidden={!expanded}>
      <p>{description}</p>
      <Copyable name="command" on:copy={toggleDropdown}>
        <p class="typo-text-small-mono command">{command}</p>
      </Copyable>
    </div>

    <slot prop={toggleDropdown} />
  </Overlay>
</div>
