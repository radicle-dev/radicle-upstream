<script>
  import { pop } from "svelte-spa-router";

  import { fallback } from "../src/identity.ts";
  import * as remote from "../src/remote.ts";
  import { session } from "../src/session.ts";
  import * as transaction from "../src/transaction.ts";

  import {
    ModalLayout,
    Transaction,
    TransactionStatusbar
  } from "../DesignSystem/Component";
  import { Button } from "../DesignSystem/Primitive";

  export let params = null;
  // TODO(xla): Can go once we get proper transaction participants.
  let identity = fallback;

  $: if (
    $session.status === remote.Status.Success &&
    $session.data.identity !== null
  ) {
    identity = $session.data.identity;
  }

  const tx = transaction.fetch(params.id);
</script>

<style>
  .transaction {
    margin: 48px 0 32px 0;
  }

  .button-row {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: 32px;
  }
</style>

<ModalLayout dataCy="page">
  <div class="transaction" data-cy="transaction">
    {#if $tx.status === remote.Status.Success}
      <!-- TODO(merle): Retrieve actual data for variant, progress & timestamp -->
      <TransactionStatusbar
        style="margin-bottom: 32px; margin-top: 96px;"
        variant="caution"
        progress={0}
        time={$tx.data.timestamp} />
      <Transaction
        tx={$tx.data}
        payer={transaction.formatPayer(identity)}
        subject={transaction.formatSubject(identity, $tx.data.messages[0])} />
    {/if}

    <div class="button-row">
      <Button
        dataCy="back-button"
        disabled={false}
        on:click={pop}
        variant="vanilla">
        Back
      </Button>
    </div>
  </div>
</ModalLayout>
