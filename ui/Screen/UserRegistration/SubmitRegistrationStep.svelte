<script>
  import * as transaction from "../../src/transaction.ts";

  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  export let handle = null;
  export let identity = null;

  export let onNextStep = null;
  export let onPreviousStep = null;

  const tx = {
    messages: [
      {
        type: transaction.MessageType.UserRegistration,
        handle,
        id: identity.id,
      },
    ],
  };
  const payer = transaction.formatPayer(identity);
  const subject = transaction.formatSubject(identity, tx.messages[0]);
</script>

<Transaction transaction={tx} {payer} {subject} />

<Flex style="margin-top: 32px;" align="right">
  <Button
    dataCy="back-button"
    disabled={false}
    on:click={onPreviousStep}
    variant="transparent"
    style="margin-right: 24px">
    Back
  </Button>
  <Button
    dataCy="submit-button"
    disabled={false}
    on:click={onNextStep}
    variant="primary">
    Submit transaction
  </Button>
</Flex>
