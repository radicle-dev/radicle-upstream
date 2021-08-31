<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { SvelteComponent } from "svelte";
  import type { Tx } from "ui/src/transaction";

  import { Icon } from "ui/DesignSystem";
  import dayjs from "dayjs";
  import { TxKind } from "ui/src/transaction";

  export let tx: Tx;

  function txIcon(tx: Tx): typeof SvelteComponent {
    switch (tx.kind) {
      case TxKind.ClaimRadicleIdentity:
        return Icon.Registered;
      case TxKind.CollectFunds:
      case TxKind.Withdraw:
        return Icon.Withdraw;
      case TxKind.CommitEnsName:
      case TxKind.Erc20Allowance:
      case TxKind.LinkEnsNameToOrg:
      case TxKind.RegisterEnsName:
      case TxKind.UpdateEnsMetadata:
        return Icon.Ethereum;
      case TxKind.SupportOnboarding:
      case TxKind.UpdateSupport:
        return Icon.TokenStreams;
      case TxKind.TopUp:
        return Icon.Topup;
      case TxKind.CreateOrg:
        return Icon.Orgs;
      case TxKind.AnchorProject:
        return Icon.Anchor;
    }
  }
</script>

<style>
  .container {
    display: flex;
    padding: 0.75rem;
    border-bottom: 1px solid var(--color-foreground-level-2);
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
  }

  .container:last-child {
    border-bottom: 0;
  }

  .left {
    display: flex;
    align-items: center;
  }

  .date {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 2rem;
    margin-right: 1rem;
    text-transform: uppercase;
  }

  .date h5 {
    color: var(--color-foreground-level-4);
    line-height: 1rem;
  }

  .date p {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="container" on:click data-cy="transaction">
  <div class="left">
    <div class="date">
      <h5>{dayjs(tx.date).format("MMM")}</h5>
      <p class="typo-text-bold">{dayjs(tx.date).format("D")}</p>
    </div>
    <svelte:component this={txIcon(tx)} />
    <p class="typo-text-bold" style="margin-left: 0.5rem">{tx.kind}</p>
  </div>
  <!-- <Label
    title={`-${price} ETH`}
    color="var(--color-negative-level-1)"
    style="color: var(--color-negative)" /> -->
</div>
