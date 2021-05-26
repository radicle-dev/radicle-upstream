<script lang="typescript">
  import { store as transactions, getShortMonth } from "ui/src/transaction";

  import TxList from "ui/DesignSystem/Component/Wallet/Transactions/TxList.svelte";

  $: pendingTxs = $transactions.filter(
    tx => tx.status === "Awaiting inclusion"
  );
  $: rejectedTxs = $transactions.filter(tx => tx.status === "Rejected");

  $: txMonthSections = $transactions.reduce((res, cur) => {
    const txMonth = new Date(cur.date).getMonth();
    const txYear = new Date(cur.date).getFullYear();

    const currentSectionIndex = res.findIndex(
      section => section.key === `${txMonth}` + `${txYear}`
    );

    if (currentSectionIndex !== -1) {
      res[currentSectionIndex] = {
        ...res[currentSectionIndex],
        items: [...res[currentSectionIndex].items, cur],
      };
    } else {
      res = [
        ...res,
        {
          key: `${txMonth}` + `${txYear}`,
          title: `${getShortMonth(new Date(cur.date))} ${txYear}`,
          items: [cur],
        },
      ];
    }
    return res;
  }, []);
</script>

<style>
  .list {
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.25rem;
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
