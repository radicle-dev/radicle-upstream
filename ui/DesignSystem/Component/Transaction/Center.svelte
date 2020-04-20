<script>
  import {
    USER_REGISTRATION,
    PROJECT_REGISTRATION
  } from "../../../../native/types.js";
  import * as remote from "../../../src/remote.ts";
  import { transactions } from "../../../src/transaction.ts";

  import { Text } from "../../Primitive";

  import Accordion from "./Accordion.svelte";

  const formatMessage = kind => {
    switch (kind) {
      case USER_REGISTRATION:
        return "User registration";
      case PROJECT_REGISTRATION:
        return "Project registration";
    }
  };

  // TODO(merle): Use actual data
  const formatTransactions = transactions => {
    return transactions.map(transaction => {
      return {
        id: transaction.id,
        message: formatMessage(Object.keys(transaction.messages[0])[0]),
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
