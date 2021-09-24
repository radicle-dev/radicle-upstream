<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { unreachable } from "ui/src/unreachable";
  import * as format from "./lib/format";

  export let params:
    | { type: "commitHash"; hash: string; onClick: () => void }
    | { type: "transactionHash"; hash: string; url: string };
</script>

<style>
  a {
    color: var(--color-foreground-level-6);
  }
</style>

{#if params.type === "commitHash"}
  <span class="typo-link" on:click={params.onClick}>
    {format.shortCommitHash(params.hash)}
  </span>
{:else if params.type === "transactionHash"}
  <a class="typo-link" href={params.url}>{format.shortEthTx(params.hash)}</a>
{:else}
  {unreachable(params)}
{/if}
