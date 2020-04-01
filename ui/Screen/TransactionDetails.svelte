<script>
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { pop } from "svelte-spa-router";

  import { Button, Title } from "../DesignSystem/Primitive";
  import { ModalLayout, Transaction } from "../DesignSystem/Component";

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
    <Title
      variant="big"
      style="margin-bottom: 24px; white-space: nowrap; overflow: hidden;
      text-overflow: ellipsis; color: var(--color-foreground-level-5)">
      <span style="color: var(--color-foreground)">Transaction</span>
      {params.id}
    </Title>
    {#await $transactions then result}
      <!-- TODO(merle): Add transaction status bar -->
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
