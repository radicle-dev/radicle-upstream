<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import type { Person } from "../../../src/source";

  import { Icon } from "../../Primitive";

  export let message: string;
  // FIXME(xla): Should be a proper type `Sha`.
  export let sha1: string;
  export let style: string = "";
  export let timestamp: string;
  export let user: Person;

  const dispatch = createEventDispatcher();
  const onSelect = () => {
    dispatch("select", sha1);
  };
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    height: 40px;
    padding: 0 12px 0 8px;
    white-space: nowrap;
    min-width: var(--content-min-width);
    border-radius: 4px;
    background-color: var(--color-secondary-level-1);
  }

  .align-left {
    display: flex;
    flex: 1;
    align-items: center;
    max-width: 64%;
  }

  .align-right {
    display: flex;
    flex: 1;
    justify-content: flex-end;
  }

  .commit-message {
    color: var(--commit-message-color, var(--color-secondary));
    text-overflow: ellipsis;
    overflow-x: hidden;
  }

  .commit-sha {
    padding: 0 8px 0 4px;
    color: var(--commit-sha-color, var(--color-secondary));
  }
</style>

<div class="container" {style} data-cy="commit-teaser">
  <div class="align-left">
    <Icon.Commit style="fill: var(--color-secondary)" />
    <!-- svelte-ignore a11y-missing-attribute -->
    <a class="commit-sha typo-text-small-mono" on:click={onSelect}>
      {sha1.substring(0, 7)}
    </a>
    <p class="commit-message typo-text-small">{message}</p>
  </div>

  <div class="align-right">
    <p
      class="typo-text-small-bold"
      style="margin-right: 8px; color: var(--color-foreground-level-6)">
      {user.name}
    </p>
    <p class="typo-text-small" style="color: var(--color-foreground-level-6)">
      {timestamp}
    </p>
  </div>
</div>
