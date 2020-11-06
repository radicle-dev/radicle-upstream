<script lang="ts">
  import { get } from "svelte/store";
  import { pop } from "svelte-spa-router";

  import * as modal from "../../../src/modal";
  import { store } from "../../../src/funding/pool";

  import { Remote, TxButton } from "../../../DesignSystem/Component";
  import { Button } from "../../../DesignSystem/Primitive";
  import { resolve } from "path";

  if ($store === null) pop();
  $: pool = get(store);

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

  h2 {
    margin-top: calc(var(--content-padding) / 2);
  }

  .from-to {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 1rem;
  }

  .subheading {
    color: var(--color-foreground-level-6);
    padding: 0.5rem;
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
        <h2>Collect incoming support</h2>

        <div class="from-to">
          <p class="typo-text-bold subheading">Incoming support</p>
          <p class="typo-text-bold subheading">-&gt;</p>
          <p class="typo-text-bold subheading">Your connected wallet</p>
        </div>
      </header>

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
