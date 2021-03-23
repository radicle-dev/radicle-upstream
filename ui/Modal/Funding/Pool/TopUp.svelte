<script lang="ts">
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import { TxButton } from "../../../DesignSystem/Component";
  import TopUp from "../../../DesignSystem/Component/Funding/Pool/TopUp.svelte";

  import * as modal from "../../../src/modal";
  import { store } from "../../../src/funding/pool";

  import Big from "big.js";

  if ($store === null) pop();

  let amount = "";
  async function onConfirmed(): Promise<void> {
    await get(store)?.topUp(Big(amount));
    modal.hide();
  }

  async function onCancel(): Promise<void> {
    modal.hide();
  }

  let disabled = true;
  let balance = Big(0);
  $: balance = $store?.getAccount()?.balance || balance;
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
  <TopUp bind:amount {balance} onBack={['Cancel', onCancel]} bind:disabled>
    <TxButton onClick={onConfirmed} {disabled} errorLabel="Failed top up">
      Confirm in your wallet
    </TxButton>
  </TopUp>
</div>
