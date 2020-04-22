<script>
  import { getClient, query } from "svelte-apollo";
  import { gql } from "apollo-boost";
  import { pop } from "svelte-spa-router";

  import { fallback } from "../src/identity.ts";
  import * as remote from "../src/remote.ts";
  import { session } from "../src/session.ts";

  import {
    ModalLayout,
    Transaction,
    TransactionStatusbar
  } from "../DesignSystem/Component";
  import { Button } from "../DesignSystem/Primitive";

  export let params = null;
  // TODO(xla): Can go once we get proper transaction participants.
  let identity = fallback;

  $: if (
    $session.status === remote.Status.Success &&
    $session.data.identity !== null
  ) {
    identity = $session.data.identity;
  }

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

  const formatMessage = kind => {
    switch (kind) {
      case "USER_REGISTRATION":
        return "User registration";
      case "PROJECT_REGISTRATION":
        return "Project registration";
    }
  };

  const formatSubject = msg => {
    return {
      name:
        msg.kind === "USER_REGISTRATION"
          ? msg.handle
          : `${identity.metadata.handle} / ${msg.projectName}`,
      kind: "user",
      avatarFallback: identity.avatarFallback,
      imageUrl: identity.metadata.avatarUrl
    };
  };

  const formatTransaction = transaction => {
    const kind = transaction.messages[0].kind;
    return {
      id: transaction.id,
      message: formatMessage(kind),
      stake: `${formatMessage(kind)} deposit`,
      subject: formatSubject(transaction.messages[0]),
      payer: {
        name: identity.metadata.displayName || identity.metadata.handle,
        kind: "user",
        avatarFallback: identity.avatarFallback,
        imageUrl: identity.metadata.avatarUrl
      }
    };
  };
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
    {#await $transactions then result}
      <!-- TODO(merle): Retrieve actual data for variant, progress & timestamp -->
      <TransactionStatusbar
        style="margin-bottom: 32px; margin-top: 96px;"
        variant="caution"
        progress={0}
        time={result.data.listTransactions.transactions[0].timestamp} />
      <Transaction
        transaction={formatTransaction(result.data.listTransactions.transactions[0])} />
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
