<script>
  import { createEventDispatcher } from "svelte";

  import { Icon } from "../../DesignSystem/Primitive";
  import Copyable from "./Copyable.svelte";

  const dispatch = createEventDispatcher();

  export let style = null;
</script>

<style>
  .info {
    margin-top: 1rem;
    background-color: var(--color-foreground-level-1);
    border-radius: 4px;
    padding: 0.5rem;
    align-items: left;
    text-align: left;
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
      dispatch('hide');
    }}>
    <Icon.CrossSmall />
  </div>
  <p style="margin-bottom: 0.75rem; color: display: flex;">
    Before you get started, you need to add this to your shell configuration.
    Not sure how?
    <a
      style="color: var(--color-foreground-level-6);"
      class="typo-link"
      href="https://radicle.xyz/#upstream-faq">
      Read more
    </a>
    .
  </p>
  <Copyable>
    <p class="typo-text-small-mono">export PATH="$HOME/.radicle/bin:$PATH"</p>
  </Copyable>
</div>
