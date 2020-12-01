<script lang="typescript">
  import type { Tx } from "../../../src/transaction";
  import { TxKind } from "../../../src/transaction";

  import Receivers from "../Funding/Pool/Receivers.svelte";

  export let tx: Tx;
  export let style = "";
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    flex-direction: column;
  }

  strong {
    font-weight: bold;
  }

  p {
    padding: 0 4rem;
  }
</style>

<div class="wrapper" {style}>
  {#if tx.kind === TxKind.Withdraw}
    <p>
      Withdraw
      <strong>{tx.amount} DAI</strong>
      from your support balance to your external wallet.
    </p>
  {:else if tx.kind === TxKind.TopUp}
    <p>Top up your support balance with <strong>{tx.amount} DAI</strong>.</p>
  {:else if tx.kind === TxKind.CollectFunds}
    <p>Collect <strong>{tx.amount} DAI</strong> from your supporters.</p>
  {:else if tx.kind === TxKind.UpdateMonthlyContribution}
    <p>
      Update your support's monthly contribution to
      <strong>{tx.amount} DAI</strong>.
    </p>
  {:else if tx.kind === TxKind.UpdateReceivers}
    <p>Updated the list of receivers by adding or removing the following:</p>
    <Receivers
      receivers={new Map(tx.receivers)}
      style="margin-top: 1.2rem"
      alignment="center" />
  {/if}
</div>
