<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as modal from "ui/src/modal";
  import type { Tx } from "ui/src/transaction";

  import TransactionModal from "../TransactionModal.svelte";
  import TransactionListItem from "./TransactionListItem.svelte";

  export let title: string;
  export let txs: Tx[];

  function onSelect(hash: string): void {
    modal.toggle(TransactionModal, () => {}, {
      transactionHash: hash,
    });
  }
</script>

<style>
  .title-row {
    display: flex;
    flex: 1;
    padding: 0.25rem 0.75rem;
    color: var(--color-foreground-level-5);
    background-color: var(--color-foreground-level-1);
    border-bottom: 1px solid var(--color-foreground-level-2);
  }

  .title-row:first-child {
    border-top-left-radius: 0.4rem;
    border-top-right-radius: 0.4rem;
  }
</style>

<h5 class="title-row">{title}</h5>
{#each txs as tx}
  <TransactionListItem on:click={() => onSelect(tx.hash)} {tx} />
{/each}
