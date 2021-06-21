<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import {
    store as transactions,
    getShortMonth,
    TxStatus,
  } from "ui/src/transaction";
  import type { Tx } from "ui/src/transaction";

  import TxList from "ui/DesignSystem/Wallet/Transactions/TxList.svelte";

  $: includedTxs = $transactions.filter(tx => tx.status === TxStatus.Included);
  $: pendingTxs = $transactions.filter(
    tx => tx.status === TxStatus.AwaitingInclusion
  );
  $: rejectedTxs = $transactions.filter(tx => tx.status === TxStatus.Rejected);

  const groupTxs = (txs: Tx[]) => {
    const sections: Array<{ key: string; title: string; items: Tx[] }> = [];
    // Sort from newest to oldest
    txs.sort((a, b) => b.date - a.date);
    for (const tx of txs) {
      const txDate = new Date(tx.date);
      const txMonth = txDate.getMonth();
      const txYear = txDate.getFullYear();
      const key = `${txYear}-${txMonth}`;
      const currentSection = sections[sections.length - 1];
      if (currentSection && currentSection.key === key) {
        currentSection.items.push(tx);
      } else {
        const title = `${getShortMonth(txDate)} ${txYear}`;
        sections.push({
          key,
          title,
          items: [tx],
        });
      }
    }
    return sections;
  };

  $: txMonthSections = groupTxs(includedTxs);
</script>

<style>
  .list {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
    margin-bottom: 1.5rem;
  }
</style>

<div>
  {#if $transactions.length > 0}
    {#if pendingTxs.length > 0}
      <div class="list">
        <TxList title="Pending transactions" txs={pendingTxs} />
      </div>
    {/if}
    {#if rejectedTxs.length > 0}
      <div class="list">
        <TxList title="Rejected transactions" txs={rejectedTxs} />
      </div>
    {/if}
    <div class="list">
      {#each txMonthSections as section}
        <TxList title={section.title} txs={section.items} />
      {/each}
    </div>
  {/if}
</div>
