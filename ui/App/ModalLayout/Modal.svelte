<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Emoji from "design-system/Emoji.svelte";

  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;

  export let emoji: string | undefined = undefined;
  export let title: string | undefined = undefined;
</script>

<style>
  .container {
    width: 37.5rem;
    background: var(--color-background);
    border-radius: 1rem;
    padding: 2rem;
    box-shadow: var(--elevation-high);
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 2rem;
    margin-bottom: 2rem;
  }
  .container:focus {
    outline: none;
  }
  h1 {
    text-align: center;
    margin-bottom: 1rem;
  }
  .description {
    color: var(--color-foreground-level-6);
    text-align: center;
    margin-bottom: 1rem;
  }
  .content {
    margin-bottom: 1rem;
    width: 100%;
  }
  .buttons {
    margin-top: 1rem;
    display: flex;
    width: 100%;
    gap: 1.5rem;
    justify-content: flex-end;
  }
  h1 {
    text-align: center;
  }
</style>

<div role="dialog" class="container" data-cy={dataCy} aria-label={title}>
  {#if emoji}
    <Emoji {emoji} size="huge" style="margin-bottom: 1rem;" />
  {/if}

  {#if title}
    <h1>{title}</h1>
  {/if}

  {#if $$slots.description}
    <p class="description">
      <slot name="description" />
    </p>
  {/if}

  <div class="content" {style}>
    <slot />
  </div>

  {#if $$slots.buttons}
    <div class="buttons">
      <slot name="buttons" />
    </div>
  {/if}
</div>
