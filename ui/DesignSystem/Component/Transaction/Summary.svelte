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
</style>

<div class="wrapper" {style}>
  {#if tx.kind === TxKind.Withdraw}
    <p>
      Withdraw
      <strong>{tx.meta.amount} DAI</strong>
      from your support balance to your external wallet.
    </p>
  {:else if tx.kind === TxKind.TopUp}
    <p>
      Top up your support balance with
      <strong>{tx.meta.amount} DAI</strong>.
    </p>
  {:else if tx.kind === TxKind.CollectFunds}
    <p>Collect <strong>{tx.meta.amount} DAI</strong> from your supporters.</p>
  {:else if tx.kind === TxKind.UpdateMonthlyContribution}
    <p>
      Update your support's monthly contribution to
      <strong>{tx.meta.amount} DAI</strong>.
    </p>
  {:else if tx.kind === TxKind.UpdateReceivers}
    <p>
      Update the list of receivers of your support to:
      <Receivers receivers={tx.meta.receivers} style="margin: 1rem 0" />
    </p>
  {/if}
</div>
