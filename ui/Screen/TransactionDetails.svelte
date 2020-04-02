<script>
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { pop } from "svelte-spa-router";

  import { Button } from "../DesignSystem/Primitive";
  import {
    ModalLayout,
    Transaction,
    TransactionStatusbar
  } from "../DesignSystem/Component";

  export let params = null;

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
          messages {
            ... on ProjectRegistrationMessage {
              kind
              orgId
              projectName
            }
            ... on UserRegistrationMessage {
              kind
              id
              handle
            }
          }
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
      ids: [params.id]
    },
    fetchPolicy: "no-cache"
  });

  const formatMessage = {
    USER_REGISTRATION: "User registration",
    ORG_REGISTRATION: "Org registration",
    ORG_UNREGISTRATION: "Org unregistration",
    PROJECT_REGISTRATION: "Project registration"
  };

  const formatTx = tx => {
    const kind = tx.messages[0].kind;
    return {
      id: tx.id,
      message: formatMessage[kind],
      stake: `${formatMessage[kind]} deposit`,
      // TODO(merle): Retrieve actual data for subject and payer
      subject: {
        name: "handle",
        kind: "user",
        avatarFallback: null,
        imageUrl: null
      },
      payer: {
        name: "handle",
        kind: "user",
        avatarFallback: null,
        imageUrl: null
      }
    };
  };
</script>

<style>
  .transaction {
    margin-top: 48px;
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
    {#await $transactions then result}
      <!-- TODO(merle): Retrieve actual data for variant, progress & timestamp -->
      <TransactionStatusbar
        style="margin-bottom: 32px; margin-top: 96px;"
        variant="caution"
        progress={0}
        time={result.data.listTransactions.transactions[0].timestamp} />
      <Transaction
        tx={formatTx(result.data.listTransactions.transactions[0])} />
    {/await}

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
