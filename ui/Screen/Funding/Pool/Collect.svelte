<script lang="ts">
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import * as modal from "../../../src/modal";
  import { store } from "../../../src/funding/pool";

  import { Remote, TxButton } from "../../../DesignSystem/Component";
  import { Button, Icon, Input } from "../../../DesignSystem/Primitive";
  import { resolve } from "path";

  if ($store === null) pop();
  $: pool = get(store);

  const disableConfirmation = false;
  async function onConfirmed(): Promise<void> {
    await get(store).collect();
    modal.hide();
    resolve();
  }

  async function onCancel(): Promise<void> {
    modal.hide();
  }
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
    width: 100%;
    padding: var(--content-padding);
    margin-bottom: 1.5rem;
    background-color: var(--color-foreground-level-1);
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.25rem;
  }
  .icon {
    height: 56px;
    width: 56px;
    border-radius: 50%;
    background-color: var(--color-primary-level-5);
    border: 2px solid #5555ff;
    display: flex;
    justify-content: center;
    align-items: center;
    margin-bottom: 1rem;
  }

  .from-to {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 1rem;
  }

  .sub-section {
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    margin-top: 1.7rem;
  }

  .sub-section p {
    color: var(--color-foreground-level-6);
  }

  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;

    border: 1px solid var(--color-foreground-level-3);
    box-sizing: border-box;
    border-radius: 4px;

    padding: 10px;
  }

  .subheading {
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
  }

  .address {
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;

    display: inline-flex;
    max-width: 80px;
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    margin-top: 2rem;
  }
</style>

<Remote store={pool.data} let:data={poolData}>
  <div class="wrapper" data-cy="send-funds-modal">
    <div data-cy="preparation-step">
      <header>
        <div class="icon">
          <Icon.ArrowUp style="fill: #5555FF" />
        </div>
        <h2>Collect incoming support</h2>

        <div class="from-to">
          <p class="typo-text-bold subheading">Incoming support</p>
          <p class="typo-text-bold subheading">-&gt;</p>
          <p class="typo-text-bold subheading">Your connected wallet</p>
        </div>
      </header>

      <div class="sub-section">
        <p class="typo-text-bold subheading">From</p>
        <div class="row">
          <p>Your incoming support ({poolData.collectableFunds} DAI)</p>
        </div>
      </div>

      <div class="sub-section">
        <p class="typo-text-bold subheading">To</p>
        <div class="row">
          <p>Your connected account</p>
          <p class="address">{get(store).getAccount().address}</p>
        </div>
      </div>

      <div class="submit">
        <Button variant="transparent" dataCy="cancel-topup" on:click={onCancel}>
          Cancel
        </Button>

        <TxButton
          title="Confirm"
          dataCy="review-transfer-button"
          onClick={onConfirmed}
          errorMessage={e => `Could not top up pool funds: ${e.message}`} />
      </div>
    </div>
  </div>
</Remote>
