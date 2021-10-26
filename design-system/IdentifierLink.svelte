<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { unreachable } from "./lib/unreachable";
  import * as format from "./lib/format";

  import CommitIcon from "./icons/Commit.svelte";
  import EthereumIcon from "./icons/Ethereum.svelte";

  export let params:
    | { type: "commitHash"; hash: string; onClick: () => void }
    | { type: "transactionHash"; hash: string; url: string };
  export let showIcon: boolean = false;
</script>

<style>
  a,
  span {
    color: var(--color-foreground-level-6);
  }
</style>

<div style="display: flex;">
  {#if params.type === "commitHash"}
    {#if showIcon}
      <CommitIcon />
    {/if}
    <span class="typo-link" on:click={params.onClick}>
      {format.shortCommitHash(params.hash)}
    </span>
  {:else if params.type === "transactionHash"}
    {#if showIcon}
      <EthereumIcon />
    {/if}
    <a class="typo-link" href={params.url}>{format.shortEthTx(params.hash)}</a>
  {:else}
    {unreachable(params)}
  {/if}
</div>
