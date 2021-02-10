<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { Variant as IllustrationVariant } from "../../src/illustration";

  import { Button, Emoji } from "../Primitive";
  import Illustration from "./Illustration.svelte";

  const dispatch = createEventDispatcher();

  export let style: string = "";
  export let illustration: IllustrationVariant = IllustrationVariant.Plant;
  export let emoji: string = "";
  export let text: string = "Nothing to see here";
  export let headerText: string = "";
  export let primaryActionText: string = "";
  export let secondaryActionText: string = "";

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
    max-width: 30rem;
    margin: 0 auto;
  }
  h3 {
    margin-top: 1.8rem;
  }
  .text {
    color: var(--color-foreground-level-6);
    margin: 1.5rem 0;
    max-width: 20rem;
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
  {#if emoji.length}
    <Emoji {emoji} size="huge" />
  {:else}
    <Illustration variant={illustration} />
  {/if}
  {#if headerText.length}
    <h3>{headerText}</h3>
  {/if}
  {#if text.length}
    <p class="text">{text}</p>
  {/if}
  {#if primaryActionText.length}
    <Button
      dataCy="primary-action"
      on:click={() => onPrimaryAction()}
      style="margin-bottom: 0.75rem;">
      {primaryActionText}
    </Button>
  {/if}
  {#if secondaryActionText.length}
    <button data-cy="secondary-action" on:click={() => onSecondaryAction()}>
      <p>{secondaryActionText}</p>
    </button>
  {/if}
</div>
