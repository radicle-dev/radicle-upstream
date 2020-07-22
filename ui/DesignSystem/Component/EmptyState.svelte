<script>
  import { createEventDispatcher } from "svelte";
  import { Button, Icon, Text } from "../Primitive";

  const dispatch = createEventDispatcher();

  export let style = null;
  export let icon = "plant";
  export let text = "Nothing to see here";
  export let primaryActionText = null;
  export let secondaryActionText = null;

  const onPrimaryAction = () => {
    dispatch("primaryAction");
  };
  const onSecondaryAction = () => {
    dispatch("secondaryAction");
  };
</script>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: calc(100vh - var(--bigheader-height));
    text-align: center;
    max-width: 20rem;
    margin: 0 auto;
  }
  button {
    color: var(--color-foreground-level-5);
    text-decoration: underline;
    cursor: pointer;
  }
  button:hover {
    color: var(--color-secondary);
  }

  button:active {
    color: var(--color-secondary-level-6);
  }

  button:focus {
    outline-style: none;
  }
</style>

<div class="empty-state" data-cy="empty-state" {style}>
  <Icon.EmptyState variant={icon} style="height: 48px; width: 48px;" />
  <Text style="margin: 1.5rem 0; color: var(--color-foreground-level-6);">
    {text}
  </Text>
  {#if primaryActionText !== null}
    <Button
      dataCy="primary-action"
      on:click={() => onPrimaryAction()}
      style="margin-bottom: 0.75rem;">
      {primaryActionText}
    </Button>
  {/if}
  {#if secondaryActionText !== null}
    <button data-cy="secondary-action" on:click={() => onSecondaryAction()}>
      <Text>{secondaryActionText}</Text>
    </button>
  {/if}
</div>
