<script lang="typescript">
  import { Copyable, Identity } from "../DesignSystem/Component";
  import { Emoji, Icon } from "../DesignSystem/Primitive";
  import TxSpinner from "../DesignSystem/Component/Transaction/Spinner.svelte";
  import Summary from "../DesignSystem/Component/Transaction/Summary.svelte";

  import { ellipsed } from "../src/style";
  import {
    emoji,
    selectedStore,
    store as txs,
    colorForStatus,
    isIncoming,
    formatDate,
    transferAmount,
    TxStatus,
    TxKind,
  } from "../src/transaction";
  import type { Tx } from "../src/transaction";

  // In reality, the transaction should never be undefined,
  // but because the only way we currently have use it here
  // is by looking it up, type-wise it can.
  let tx: Tx | undefined = undefined;

  $: tx = $txs.find(x => x.hash === $selectedStore);

  $: statusColor = colorForStatus(tx?.status || TxStatus.AwaitingInclusion);
  $: transferedAmount = tx ? transferAmount(tx) : undefined;
  $: incoming = tx ? isIncoming(tx) : false;
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    flex-direction: column;
    padding: var(--content-padding);
    width: 40.625rem;
    background: var(--color-background);
    border-radius: 1rem;
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
    border-radius: 0.5rem;
  }

  .from-to:not(.incoming) {
    flex-direction: column-reverse;
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
    padding: 1rem;
    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 0.5rem;
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
  {#if tx}
    <header>
      <Emoji emoji={emoji(tx)} size="huge" />
      <h1>{tx.kind}</h1>
      <Summary {tx} style="margin-top: 1.5rem" />
      {#if tx.kind !== TxKind.ClaimRadicleIdentity}
        <div class="from-to" class:incoming>
          <div>
            <p class="typo-text-bold" style="margin-bottom: 7px">
              Radicle Pool
            </p>
            <Copyable
              showIcon={false}
              styleContent={false}
              copyContent={tx.to}
              notificationText="Address copied to the clipboard">
              <p class="address typo-text">{tx.to || "n/a"}</p>
            </Copyable>
          </div>

          <div class="arrow">
            <Icon.ArrowDown />
          </div>

          <Identity address={tx.from} />
        </div>
      {/if}
    </header>

    <div class="content">
      {#if transferedAmount}
        <div class="section">
          <div class="row">
            <p>Amount</p>
            <p class="typo-semi-bold">
              {#if incoming}
                {transferedAmount} DAI
              {:else}
                - {transferedAmount} DAI
              {/if}
            </p>
          </div>
        </div>
      {/if}

      <div class="section">
        <div class="row">
          <p>Transaction ID</p>
          <p class="typo-text-small-mono">
            <Copyable
              showIcon={false}
              styleContent={false}
              copyContent={tx.hash}
              notificationText="Transaction ID copied to the clipboard">
              {ellipsed(tx.hash, 12)}
            </Copyable>
          </p>
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
  {/if}
</div>
