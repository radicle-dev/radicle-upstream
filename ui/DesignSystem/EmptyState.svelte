<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import * as Style from "ui/src/style";

  import Button from "./Button.svelte";
  import Emoji from "./Emoji.svelte";
  import Tooltip from "./Tooltip.svelte";

  const dispatch = createEventDispatcher();

  export let style: string = "";
  export let emoji: string = "";
  export let text: string = "Nothing to see here";
  export let headerText: string = "";
  export let primaryActionText: string = "";
  export let secondaryActionText: string = "";
  export let primaryActionDisabled = false;
  export let primaryActionTooltipMessage: string | undefined = undefined;

  $: tooltipMessage = primaryActionDisabled ? primaryActionTooltipMessage : "";

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
    color: var(--color-primary);
  }

  button:active {
    color: var(--color-primary-level-6);
  }

  button:focus {
    outline-style: none;
  }
</style>

<div class="empty-state" data-cy="empty-state" {style}>
  {#if emoji.length}
    <Emoji {emoji} size="huge" />
  {/if}
  {#if headerText.length}
    <h3>{headerText}</h3>
  {/if}
  {#if text.length}
    <p class="text">{text}</p>
  {/if}
  {#if primaryActionText.length}
    <Tooltip value={tooltipMessage} position={Style.CSSPosition.Bottom}>
      <Button
        disabled={primaryActionDisabled}
        dataCy="primary-action"
        on:click={() => onPrimaryAction()}>
        {primaryActionText}
      </Button>
    </Tooltip>
  {/if}
  {#if secondaryActionText.length}
    <button data-cy="secondary-action" on:click={() => onSecondaryAction()}>
      <p>{secondaryActionText}</p>
    </button>
  {/if}
  <slot />
</div>
