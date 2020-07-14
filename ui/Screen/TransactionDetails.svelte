<script>
  import { getContext } from "svelte";

  import { fallback } from "../src/identity.ts";
  import * as transaction from "../src/transaction.ts";

  import { ModalLayout, Remote, Transaction } from "../DesignSystem/Component";

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
</style>

<ModalLayout dataCy="page">
  <div class="transaction" data-cy="transaction">
    <Remote {store} let:data={tx}>
      <Transaction
        transaction={tx}
        {payer}
        transactionDeposits={session.transactionDeposits} />
    </Remote>

  </div>
</ModalLayout>
