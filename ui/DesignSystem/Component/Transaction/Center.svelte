<script>
  import * as remote from "../../../src/remote.ts";
  import { formatMessage, transactions } from "../../../src/transaction.ts";

  import { Text } from "../../Primitive";

  import Accordion from "./Accordion.svelte";

  // TODO(merle): Use actual data.
  const formatTransactions = txs => {
    return txs.map(tx => {
      return {
        id: tx.id,
        message: formatMessage(tx.messages[0]),
        state: "pending",
        progress: 0
      };
    });
  };
</script>

{#if $transactions.status === remote.Status.Success}
  {#if $transactions.data.length > 0}
    <Accordion
      transactions={formatTransactions($transactions.data)}
      style="position: absolute; bottom: 32px; right: 32px;" />
  {/if}
{:else if $transactions.status === remote.Status.Error}
  <Text>Transactions errored</Text>
{/if}
