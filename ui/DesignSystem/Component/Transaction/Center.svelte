<script>
  import { transactions } from "../../../src/transaction.ts";

  import { Text } from "../../Primitive";
  import Accordion from "./Accordion.svelte";

  import {
    USER_REGISTRATION,
    PROJECT_REGISTRATION
  } from "../../../../native/types.js";

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
        message: formatMessage(transaction.messages[0].kind),
        state: "pending",
        progress: 0
      };
    });
  };

  $: console.log($transactions);
</script>

{#if $transactions.status === 'SUCCESS'}
  {#if $transactions.data.length > 0}
    <Accordion
      transactions={formatTransactions($transactions.data)}
      style="position: absolute; bottom: 32px; right: 32px;" />
  {/if}
{:else if $transactions.status === 'ERROR'}
  <Text>Transactions errored</Text>
{/if}
