<script>
  import { getContext } from "svelte";
  import { pop } from "svelte-spa-router";

  import { fallback } from "../src/identity.ts";
  import * as transaction from "../src/transaction.ts";

  import {
    ModalLayout,
    Remote,
    Transaction,
    TransactionStatusbar,
  } from "../DesignSystem/Component";
  import { Button } from "../DesignSystem/Primitive";

  export let params = null;

  const session = getContext("session");
  // TODO(xla): Can go once we get proper transaction participants.
  let identity = fallback;

  if (session.identity !== null) {
    identity = session.identity;
  }

  $: payer = transaction.formatPayer(identity);
  $: store = transaction.fetch(params.id);
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
    <Remote {store} let:data={tx}>
      <TransactionStatusbar
        progress={transaction.iconProgress(tx.state)}
        text={transaction.statusText(tx.state)}
        variant={transaction.iconState(tx.state)}
        style="margin-bottom: 32px; margin-top: 96px;" />
      <Transaction
        transaction={tx}
        {payer}
        transactionCosts={session.transactionCosts} />
    </Remote>

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
