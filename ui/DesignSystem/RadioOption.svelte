<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { slide } from "svelte/transition";

  export let active: boolean = false;
  export let title: string | undefined = undefined;
  export let dataCy: string | undefined = undefined;
</script>

<style>
  .option {
    border: 0.0625rem solid var(--color-foreground-level-3);
    margin-bottom: 1rem;
    border-radius: 0.5rem;
  }

  .option.active {
    background-color: var(--color-foreground-level-1);
  }

  .option:hover {
    outline: none;
    background-color: var(--color-foreground-level-1);
  }

  .header {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-height: 4.5rem;
    cursor: pointer;
    user-select: none;
  }

  .body {
    background-color: var(--color-foreground-level-1);
    padding: 0 1rem 1rem 1rem;
    border-radius: 0 0 0.5rem 0.5rem;
  }
</style>

<div
  class="option"
  class:button-transition={!active}
  class:active
  data-cy={dataCy}>
  <div class="header" on:click>
    {#if title}
      <p class="typo-text-bold" style="color: var(--color-foreground-level-6)">
        {title}
      </p>
    {/if}
    <slot name="option-header" />
  </div>
  {#if active}
    <div class="body" in:slide>
      <slot name="option-body" />
    </div>
  {/if}
</div>
