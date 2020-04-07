<script>
  import Accordion from "./Accordion.svelte";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  const GET_TRANSACTIONS = gql`
    query Query($ids: [ID!]!) {
      listTransactions(ids: $ids) {
        transactions {
          id
          timestamp
          messages {
            ... on ProjectRegistrationMessage {
              kind
            }
            ... on UserRegistrationMessage {
              kind
            }
          }
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
    fetchPolicy: "no-cache"
  });

  const formatMessage = {
    USER_REGISTRATION: "User registration",
    PROJECT_REGISTRATION: "Project registration"
  };

  // TODO(merle): Use actual data
  const formatTransactions = transactions => {
    return transactions.map(transaction => {
      return {
        id: transaction.id,
        message: formatMessage[transaction.messages[0].kind],
        state: "pending",
        progress: 0
      };
    });
  };
</script>

{#await $transactions then result}
  {#if result.data.listTransactions.transactions.length > 0}
    <Accordion
      transactions={formatTransactions(result.data.listTransactions.transactions)}
      style="position: absolute; bottom: 32px; right: 32px;" />
  {/if}
{/await}
