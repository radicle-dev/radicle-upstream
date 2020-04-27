<script>
  import { pop } from "svelte-spa-router";

  import { fallback } from "../../src/identity.ts";
  import * as project from "../../src/project.ts";
  import * as remote from "../../src/remote.ts";
  import { session } from "../../src/session.ts";
  import * as transaction from "../../src/transaction.ts";

  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  export let projectId = null;
  export let registrarHandle = null;
  export let projectName = null;
  export let projectDescription = null;

  const onSubmitTransaction = () => {
    project.register(registrarHandle, projectName, projectId);

    pop();
  };

  let identity = fallback;

  if (
    $session.status === remote.Status.Success &&
    $session.data.identity !== null
  ) {
    identity = $session.data.identity;
  }

  const tx = {
    messages: [
      {
        type: transaction.MessageType.ProjectRegistration,
        cocoId: projectId,
        orgId: registrarHandle,
        projectName: projectName,
        projectDescription: projectDescription
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
