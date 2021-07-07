<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { get } from "svelte/store";

  import { TxButton } from "ui/DesignSystem";
  import TopUp from "ui/DesignSystem/Funding/Pool/TopUp.svelte";

  import * as modal from "ui/src/modal";
  import { store } from "ui/src/funding/pool";
  import { accountBalancesStore } from "ui/src/wallet";
  import * as ethereum from "ui/src/ethereum";

  import Big from "big.js";

  let amount = "";
  async function onConfirmed(): Promise<void> {
    await get(store)?.topUp(Big(amount));
    modal.hide();
  }

  async function onCancel(): Promise<void> {
    modal.hide();
  }

  let disabled = true;
  $: balance = ethereum.toBaseUnit($accountBalancesStore.dai || Big(0));
</script>

<style>
  .wrapper {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-direction: column;

    text-align: center;

    padding: var(--content-padding);
    background: var(--color-background);
    border-radius: 0.5rem;

    width: 37.5rem;
  }
</style>

<div class="wrapper" data-cy="top-up-modal">
  <TopUp bind:amount {balance} onBack={["Cancel", onCancel]} bind:disabled>
    <TxButton onClick={onConfirmed} {disabled} errorLabel="Failed top up">
      Confirm in your wallet
    </TxButton>
  </TopUp>
</div>
