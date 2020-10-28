<script lang="typescript">
  import { get } from "svelte/store";
  import TxSpinner from "../DesignSystem/Component/Transaction/Spinner.svelte";

  import {
    selectedStore,
    store as transactionsStore,
    colorForStatus,
    TxKind,
  } from "../src/transaction";
  import type { Tx } from "../src/transaction";

  let tx: Tx = undefined;

  transactionsStore.subscribe(
    txs => (tx = txs.find(x => x.hash === get(selectedStore)))
  );

  $: statusColor = colorForStatus(tx.status);
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    flex-direction: column;
    padding: var(--content-padding);
    width: 650px;
    background: var(--color-background);
    border-radius: 0.5rem;
  }

  header {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-around;
    flex-basis: 100%;

    background: var(--color-foreground-level-1);
    border: 1px solid #ebeff3;
    box-sizing: border-box;
    border-radius: 8px;

    padding: var(--content-padding);
    text-align: center;

    color: var(--color-foreground-level-6);
  }

  header .from-to {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 1rem;
  }

  header .from-to .arrow {
    color: var(--color-foreground-level-5);
    padding: 0 1rem;
  }

  header .icon {
    height: 56px;
    width: 56px;
    border-radius: 50%;
    background-color: var(--color-primary-level-5);
    border: 2px solid #5555ff;
    display: flex;
    justify-content: center;
    align-items: center;
    margin-bottom: 1rem;
  }

  header .date {
    color: var(--color-foreground-level-4);
    margin-top: 1rem;
  }

  .content .section {
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    margin-top: 1.7rem;
    padding: 16px;
    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 4px;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  .content .section .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .content .section .row + .row {
    margin-top: 1.5rem;
  }
</style>

<div class="wrapper">
  <header>
    <h2>{tx.inner.kind}</h2>
    <div class="from-to">
      <p class="typo-text-bold subheading">
        {tx.inner.kind === TxKind.CollectFunds ? 'Incoming support' : 'Your connected wallet'}
      </p>
      <p class="typo-text-bold subheading arrow">-&gt;</p>
      <p class="typo-text-bold subheading">
        {tx.inner.kind === TxKind.CollectFunds ? 'Your connected wallet' : 'Outgoing support'}
      </p>
    </div>
    <p class="typo-text date">{new Date(tx.date).toUTCString()}</p>
  </header>

  <div class="content">
    {#if !!tx.inner.amount}
      <div class="section">
        <div class="row">
          <p>Amount</p>
          <p class="typo-semi-bold">{tx.inner.amount} DAI</p>
        </div>
      </div>
    {/if}

    <div class="section">
      <div class="row">
        <p>Transaction ID</p>
        <p class="typo-semi-bold">{tx.hash.slice(0, 20)}</p>
      </div>
      <div class="row">
        <p>Status</p>
        <div class="row">
          <TxSpinner style="width: 14px; height: 14px;" status={tx.status} />
          <p style="margin-left: 7px; color: {statusColor}">{tx.status}</p>
        </div>
      </div>
    </div>
  </div>
</div>
