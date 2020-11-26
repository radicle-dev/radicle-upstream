<script lang="typescript">
  import { get } from "svelte/store";
  import {
    Copyable,
    Dai,
    Identity,
    Illustration,
  } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";
  import TxSpinner from "../DesignSystem/Component/Transaction/Spinner.svelte";
  import Summary from "../DesignSystem/Component/Transaction/Summary.svelte";

  import {
    selectedStore,
    store as transactionsStore,
    colorForStatus,
    isIncoming,
    formatDate,
    TxKind,
  } from "../src/transaction";
  import type { Tx } from "../src/transaction";
  import { Variant as IllustrationVariant } from "../src/illustration";

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

    box-sizing: border-box;
    text-align: center;

    color: var(--color-foreground-level-6);
  }

  h1 {
    margin-top: 1.5rem;
  }

  .from-to {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-around;

    width: 100%;
    padding: 1.5rem 0;

    border: 1px solid var(--color-foreground-level-2);
    background-color: var(--color-foreground-level-1);

    margin-top: 1.5rem;
    border-radius: 4px;
  }

  .from-to .arrow {
    padding: 0.7rem 0;
  }

  .from-to .address {
    color: var(--color-foreground-level-6);
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
    <Illustration variant={IllustrationVariant.Purse} />
    <h1>{tx.kind}</h1>
    <Summary {tx} style="margin-top: 1.5rem" />
    <div class="from-to">
      <p class="typo-text-bold subheading">
        <!-- TODO(nuno): DRY this -->
        {#if isIncoming(tx)}
          <span class="address typo-text">
            <Copyable
              showIcon={false}
              styleContent={false}
              copyContent={tx.to}
              notificationText="Address copied to the clipboard">
              {tx.to || 'n/a'}
            </Copyable>
          </span>
        {:else}
          <Identity />
        {/if}
      </p>
      <div class="arrow">
        <Icon.ArrowDown />
      </div>
      <p class="typo-text-bold subheading">
        {#if isIncoming(tx)}
          <Identity />
        {:else}
          <span class="address typo-text">
            <Copyable
              showIcon={false}
              styleContent={false}
              copyContent={tx.to}
              notificationText="Address copied to the clipboard">
              {tx.to || 'n/a'}
            </Copyable>
          </span>
        {/if}
      </p>
    </div>
  </header>

  <div class="content">
    {#if tx.kind !== TxKind.UpdateReceivers}
      <div class="section">
        <div class="row">
          <p>Amount</p>
          <p class="typo-semi-bold">
            <Dai variant={isIncoming(tx) ? 'regular' : 'negative'}>
              <span style="color(--color-negative)">{tx.amount}</span>
            </Dai>
          </p>
        </div>
      </div>
    {/if}

    <div class="section">
      <div class="row">
        <p>Transaction ID</p>
        <p class="typo-text-small-mono">{tx.hash.slice(0, 20)}</p>
      </div>
      <div class="row">
        <p>Status</p>
        <div class="row">
          <TxSpinner style="width: 14px; height: 14px;" status={tx.status} />
          <p style="margin-left: 7px; color: {statusColor}">{tx.status}</p>
        </div>
      </div>
      <div class="row">
        <p>Timestamp</p>
        <p>{formatDate(new Date(tx.date))}</p>
      </div>
    </div>
  </div>
</div>
