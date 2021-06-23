<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { osModifierKey } from "ui/src/hotkeys";

  export let hint = "";
  export let noModifier: boolean = false;
  export let style = "";

  const addModifier = hint.length === 1 && !noModifier && hint !== "↵";
</script>

<style>
  .hint-container {
    display: flex;
  }

  .hint {
    background-color: var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
    padding: 0.125rem 0.375rem;
    border-radius: 0.5rem;
  }

  .hint:first-child {
    margin-right: 0.25rem;
  }
</style>

<div class="hint-container" {style}>
  {#if addModifier}
    <p class="typo-text hint">{osModifierKey}</p>
  {/if}
  <p class="typo-text hint">{hint}</p>
</div>
