<script>
  import { pop } from "svelte-spa-router";

  import { fallback } from "../../src/identity.ts";
  import * as project from "../../src/project.ts";
  import * as remote from "../../src/remote.ts";
  import { session } from "../../src/session.ts";
  import * as transaction from "../../src/transaction.ts";

  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  const onSubmitTransaction = () => {
    project.register({
      name: "upstream",
      orgId: "monadic",
      cocoId: "upstream@123abcd.git"
    });
    pop();
  };

  let identity = fallback;
  let handle = null;

  if (
    $session.status === remote.Status.Success &&
    $session.data.identity !== null
  ) {
    identity = $session.data.identity;
    handle = $session.data.identity.metadata.handle;
  }

  const tx = {
    messages: [
      {
        type: transaction.MessageType.UserRegistration,
        handle,
        id: identity.id
      }
    ]
  };
  const payer = transaction.formatPayer(identity);
  const subject = transaction.formatSubject(identity, tx.messages[0]);
</script>

<Transaction transaction={tx} {payer} {subject} />

<Flex style="margin-top: 32px;" align="right">
  <Button
    dataCy="cancel-button"
    variant="transparent"
    on:click={pop}
    style="margin-right: 24px;">
    Cancel
  </Button>
  <Button dataCy="next-button" on:click={onSubmitTransaction} variant="primary">
    Submit transaction
  </Button>
</Flex>
