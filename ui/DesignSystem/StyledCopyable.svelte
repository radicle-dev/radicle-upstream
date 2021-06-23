<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import Copyable from "./Copyable.svelte";
  import Hoverable from "./Hoverable.svelte";

  export let style: string = "";
  export let dataCy: string | null = null;
  export let value: string = "";
  export let copyContent: string = value;
  export let notificationText: string = "Copied to your clipboard";
  export let truncate: boolean = false;
  export let expandable: boolean = true;
  export let showIcon: boolean = true;

  const [head, tail] = value.split(/(.{8}).*(.{8})/).filter(Boolean);

  let hover = false;
</script>

<style>
  .wrapper {
    display: flex;
    position: relative;
  }
</style>

<Hoverable bind:hovering={hover}>
  <div class="wrapper" {style} data-cy={dataCy}>
    <Copyable
      style="align-items: center; color: var(--color-foreground-level-6)"
      {copyContent}
      {notificationText}
      styleContent={hover}
      {showIcon}>
      {#if !truncate || (expandable && hover)}
        <p class="typo-text-mono">{value}</p>
      {:else}
        <p class="typo-text-mono">{head}…{tail}</p>
      {/if}
    </Copyable>
  </div>
</Hoverable>
