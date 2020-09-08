<script>
  import { getContext } from "svelte";
  import { querystring } from "svelte-spa-router";

  import { parseQueryString } from "../src/path.ts";
  import * as transaction from "../src/transaction.ts";

  import { ModalLayout, Remote, Transaction } from "../DesignSystem/Component";

  export let params = null;

  const session = getContext("session");

  const getPlayer = tx => {
    return transaction.getPayer(tx.messages[0], session);
  };

  $: store = transaction.fetch(params.id);
  $: viewerAccountId = parseQueryString($querystring).viewerAccountId;
</script>

<style>
  .transaction {
    margin: 48px 0 32px 0;
  }
</style>

<ModalLayout dataCy="page">
  <div class="transaction" data-cy="transaction">
    <Remote {store} let:data={tx}>
      <Transaction transaction={tx} payer={getPlayer(tx)} {viewerAccountId} />
    </Remote>
  </div>
</ModalLayout>
