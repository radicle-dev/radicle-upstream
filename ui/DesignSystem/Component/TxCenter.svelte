<script>
  import TxAccordion from "./TxAccordion.svelte";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  // TODO(merle): Query for messages
  const GET_TRANSACTIONS = gql`
    query Query($ids: [ID!]!) {
      listTransactions(ids: $ids) {
        transactions {
          id
          state {
            ... on Applied {
              block
            }
          }
          timestamp
        }
        thresholds {
          confirmation
          settlement
        }
      }
    }
  `;

  const client = getClient();
  const transactions = query(client, {
    query: GET_TRANSACTIONS,
    variables: {
      ids: []
    },
    fetchPolicy: "network-only"
  });

  // TODO(merle): Use actual data
  const formatTxs = txs => {
    return txs.map(tx => {
      return {
        id: tx.id,
        message: "User registrsation",
        state: "pending",
        progress: 0
      };
    });
  };
</script>

{#await $transactions then result}
  {console.log(result)}
  {#if result.data.listTransactions.transactions.length > 0}
    <TxAccordion
      transactions={formatTxs(result.data.listTransactions.transactions)}
      style="position: absolute; bottom: 32px; right: 32px;" />
  {/if}
{/await}
