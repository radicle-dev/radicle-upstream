<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Emoji } from "ui/DesignSystem";
  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;
  export let emoji: string | undefined = undefined;
  export let title: string | undefined = undefined;
  export let desc: string | undefined = undefined;
</script>

<style>
  .container {
    width: 37.5rem;
    background: var(--color-background);
    border-radius: 1rem;
    padding: 2rem;
    box-shadow: var(--color-shadows);
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .container:focus {
    outline: none;
  }
  .content {
    width: 100%;
  }

  .desc {
    margin-bottom: 1.5rem;
    color: var(--color-foreground-level-6);
    text-align: center;
  }
</style>

<div class="container" data-cy={dataCy}>
  {#if emoji}
    <Emoji {emoji} size="huge" style="margin-bottom: 1rem;" />
  {/if}
  {#if title}
    <h1 style="margin-bottom: 1rem">{title}</h1>
  {/if}
  <p class="desc">
    {desc}
  </p>
  <div class="content" {style}>
    <slot />
  </div>
</div>
