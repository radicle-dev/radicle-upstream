<script lang="typescript">
  import { get } from "svelte/store";
  import { Copyable, Dai } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";
  import TxSpinner from "../DesignSystem/Component/Transaction/Spinner.svelte";

  import { displayAddress } from "../src/funding/pool";
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
    margin: 2rem auto;
  }

  header .from-to .arrow {
    color: var(--color-foreground-level-5);
    padding: 0 1rem;
  }

  header .from-to .address {
    color: var(--color-foreground-level-5);
    font-size: 14px;
  }

  header .date {
    color: var(--color-foreground-level-4);
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
    <h2>{tx.kind}</h2>
    <div class="from-to">
      <p class="typo-text-bold subheading">
        {tx.kind === TxKind.CollectFunds ? 'Incoming support' : 'Your connected wallet'}
        <br />
        <span class="address">
          <Copyable
            showIcon={false}
            styleContent={false}
            copyContent={tx.from}
            notificationText="Address copied to the clipboard">
            {displayAddress(tx.from)}
          </Copyable>
        </span>
      </p>
      <p class="typo-text-bold subheading arrow">-&gt;</p>
      <p class="typo-text-bold subheading">
        {tx.kind === TxKind.CollectFunds ? 'Your connected wallet' : 'Outgoing support'}
        <br />
        <span class="address">
          <Copyable
            showIcon={false}
            styleContent={false}
            copyContent={tx.to}
            notificationText="Address copied to the clipboard">
            {tx.to ? displayAddress(tx.to) : 'n/a'}
          </Copyable>
        </span>
      </p>
    </div>
    <p class="typo-text date">{new Date(tx.date).toUTCString()}</p>
  </header>

  <div class="content">
    <div class="section">
      {#if !!tx.meta.amount}
        <div class="row">
          <p>Amount</p>
          <p class="typo-semi-bold">
            <Dai variant={'negative'}>{tx.meta.amount}</Dai>
          </p>
        </div>
      {/if}
      <div class="row">
        <p>Gas used</p>
        <p class="typo-semi-bold cost">
          - {tx.gas.used ? `${tx.gas.used} gwei` : 'Not yet known'}
        </p>
      </div>
      <div class="row">
        <p>Gas limit</p>
        <p class="typo-semi-bold cost">- {tx.gas.limit} gwei</p>
      </div>
      <div class="row">
        <p>Gas price</p>
        <p class="typo-semi-bold cost">- {tx.gas.price} gwei</p>
      </div>
    </div>

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
