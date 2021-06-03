<script>
  import { createEventDispatcher } from "svelte";

  import { Icon } from "../../DesignSystem/Primitive";
  import Copyable from "./Copyable.svelte";
  import Hoverable from "./Hoverable.svelte";

  const dispatch = createEventDispatcher();

  export let style = null;
  let hover = false;
</script>

<style>
  .info {
    margin-top: 1rem;
    background-color: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    padding: 0.5rem;
    align-items: left;
    text-align: left;
  }

  .description {
    margin-bottom: 0.75rem;
    color: var(--color-foreground-level-6);
  }

  .close-hint-button {
    float: right;
    cursor: pointer;
  }
</style>

<div class="info" {style} data-cy="remote-helper-hint">
  <div
    data-cy="close-hint-button"
    class="close-hint-button"
    on:click={() => {
      dispatch("hide");
    }}>
    <Icon.CrossSmall />
  </div>
  <p class="description">
    To publish code to Radicle, you need to add this to your shell configuration
    file. Not sure how?
    <a
      style="color: var(--color-foreground-level-5);"
      class="typo-link"
      href="https://docs.radicle.xyz/docs/getting-started#configuring-your-system">
      Read more
    </a>
  </p>
  <Hoverable bind:hovering={hover}>
    <Copyable showIcon={hover} styleContent={hover}>
      <p
        class="typo-text-small-mono"
        style="color: var(--color-foreground-level-6)">
        export PATH="$HOME/.radicle/bin:$PATH"
      </p>
    </Copyable>
  </Hoverable>
</div>
