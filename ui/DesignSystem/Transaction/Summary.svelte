<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Tx } from "ui/src/transaction";
  import { TxKind } from "ui/src/transaction";

  import Receivers from "ui/DesignSystem/Funding/Pool/Receivers.svelte";

  export let tx: Tx;
  export let style = "";
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    flex-direction: column;
    padding: 0 2rem;
  }

  strong {
    font-weight: bold;
  }

  p {
    padding: 0 4rem;
  }
</style>

<div class="wrapper" {style}>
  {#if tx.kind === TxKind.SupportOnboarding}
    {#if tx.receivers.length === 0}
      <p>
        Top up
        <strong>{tx.topUp} DAI</strong>. You haven’t set any receivers. As soon
        as you do, money will begin streaming to them at a rate of
        <strong>{tx.budget} DAI</strong>
        per week.
      </p>
    {:else}
      <p>
        Top up
        <strong>{tx.topUp} DAI</strong>
        and stream
        <strong>{tx.budget} DAI</strong>
        per week to these users:
      </p>
      <Receivers
        receivers={new Map(tx.receivers)}
        style="margin-top: 1.2rem"
        alignment="center" />
    {/if}
  {:else if tx.kind === TxKind.UpdateSupport}
    {#if tx.receivers.length === 0}
      <p>
        Stream
        <strong>{tx.amount} DAI</strong>
        per week. You haven’t set any receivers. As soon as you do, money will begin
        streaming to them at this rate.
      </p>
    {:else}
      <p>Stream <strong>{tx.amount} DAI</strong> per week to these users:</p>
      <Receivers
        receivers={new Map(tx.receivers)}
        style="margin-top: 1.2rem"
        alignment="center" />
    {/if}
  {/if}
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
  {/if}
</div>
