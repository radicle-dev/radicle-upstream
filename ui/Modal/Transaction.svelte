<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import dayjs from "dayjs";
  import Big from "big.js";

  import { Copyable, Icon, Identity, Modal } from "ui/DesignSystem";
  import TxSpinner from "ui/DesignSystem/Transaction/Spinner.svelte";
  import Summary from "ui/DesignSystem/Transaction/Summary.svelte";

  import type { Tx } from "ui/src/transaction";
  import * as format from "ui/src/format";
  import {
    TxKind,
    TxStatus,
    colorForStatus,
    selectedStore,
    store as txs,
  } from "ui/src/transaction";

  // In reality, the transaction should never be undefined,
  // but because the only way we currently have use it here
  // is by looking it up, type-wise it can.
  let tx: Tx | undefined = undefined;

  $: tx = $txs.find(x => x.hash === $selectedStore);

  $: statusColor = colorForStatus(tx?.status || TxStatus.AwaitingInclusion);
  $: transferedAmount = tx ? transferAmount(tx) : undefined;
  $: incoming = tx ? isIncoming(tx) : false;

  export function isIncoming(tx: Tx): boolean {
    switch (tx.kind) {
      case TxKind.CollectFunds:
      case TxKind.Withdraw:
        return true;

      case TxKind.AnchorProject:
      case TxKind.ClaimRadicleIdentity:
      case TxKind.CommitEnsName:
      case TxKind.CreateOrg:
      case TxKind.Erc20Allowance:
      case TxKind.LinkEnsNameToOrg:
      case TxKind.RegisterEnsName:
      case TxKind.SupportOnboarding:
      case TxKind.TopUp:
      case TxKind.UpdateEnsMetadata:
      case TxKind.UpdateSupport:
        return false;
    }
  }

  function showPoolCard(kind: TxKind): boolean {
    return !(
      kind === TxKind.AnchorProject ||
      kind === TxKind.ClaimRadicleIdentity ||
      kind === TxKind.CommitEnsName ||
      kind === TxKind.CreateOrg ||
      kind === TxKind.LinkEnsNameToOrg ||
      kind === TxKind.RegisterEnsName ||
      kind === TxKind.UpdateEnsMetadata
    );
  }

  function emoji(tx: Tx): string {
    switch (tx.kind) {
      case TxKind.AnchorProject:
        return "🏖️";
      case TxKind.CreateOrg:
        return "🎪";
      case TxKind.ClaimRadicleIdentity:
        return "🧦";
      case TxKind.CommitEnsName:
      case TxKind.RegisterEnsName:
        return "📇";
      case TxKind.UpdateEnsMetadata:
        return "📋";
      case TxKind.LinkEnsNameToOrg:
        return "🔗";
      case TxKind.CollectFunds:
      case TxKind.Withdraw:
      case TxKind.Erc20Allowance:
      case TxKind.SupportOnboarding:
      case TxKind.TopUp:
      case TxKind.UpdateSupport:
        return "👛";
    }
  }

  // The amount the `tx` transfers. `undefined` when not applicable.
  export function transferAmount(tx: Tx): Big | undefined {
    switch (tx.kind) {
      case TxKind.CollectFunds:
      case TxKind.Withdraw:
      case TxKind.TopUp:
        return Big(tx.amount);
      case TxKind.SupportOnboarding:
        return Big(tx.topUp);
      default:
        return undefined;
    }
  }
</script>

<style>
  header {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-around;
    flex-basis: 100%;

    box-sizing: border-box;
    text-align: center;

    color: var(--color-foreground-level-6);
    margin-bottom: 1.5rem;
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
    padding: 0.5rem 0;
  }

  .from-to .address {
    color: var(--color-foreground-level-6);
  }

  .section {
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    margin-bottom: 1.5rem;
    padding: 1rem;
    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 0.5rem;
  }

  .section:last-child {
    margin-bottom: 0;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.375rem;
  }
  .row:last-child {
    padding-bottom: 0;
  }
</style>

<Modal
  emoji={tx && emoji(tx)}
  title={tx && tx.kind}
  dataCy="transaction-summary">
  {#if tx}
    <header>
      <Summary {tx} />
      {#if showPoolCard(tx.kind)}
        <div class="from-to" class:incoming>
          <div>
            <p class="typo-text-bold" style="margin-bottom: 0.5rem">
              Radicle Pool
            </p>
            <Copyable name="pool address" clipboardContent={tx.to}>
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
            <Copyable name="transaction hash" clipboardContent={tx.hash}>
              {format.shortEthTx(tx.hash)}
            </Copyable>
          </p>
        </div>
        <div class="row">
          <p>Status</p>
          <div class="row" data-cy="transaction-status">
            <TxSpinner style="width: 14px; height: 14px;" status={tx.status} />
            <p style="margin-left: 0.5rem; color: {statusColor}">{tx.status}</p>
          </div>
        </div>
        <div class="row">
          <p>Timestamp</p>
          <p>
            {dayjs(tx.date).format("HH:mm:ss [on] D MMMM YYYY")}
          </p>
        </div>
      </div>
    </div>
  {/if}
</Modal>
