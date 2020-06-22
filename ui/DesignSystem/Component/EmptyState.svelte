<script>
  import Icon from "./EmptyState/Icon.svelte";
  import { Button, Text } from "../Primitive";

  export let icon = "tent";
  export let text = "Nothing to see here";
  export let mainCtaText = "Go back";
  export let mainDataCy = null;
  export let mainCtaAction = null;
  export let secondaryCtaText = "Go back again";
  export let secondaryCtaAction = null;
  export let secondaryDataCy = null;
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

<div class="empty-state">
  <Icon variant={icon} />
  <Text style="margin: 1.5rem 0; color: var(--color-foreground-level-6);">
    {text}
  </Text>
  <Button
    DataCy={mainDataCy}
    on:click={mainCtaAction}
    style="margin-bottom: 0.75rem;">
    {mainCtaText}
  </Button>
  {#if secondaryCtaText !== ''}
    <button data-cy={secondaryDataCy} on:click={secondaryCtaAction}>
      <Text>{secondaryCtaText}</Text>
    </button>
  {/if}
</div>
