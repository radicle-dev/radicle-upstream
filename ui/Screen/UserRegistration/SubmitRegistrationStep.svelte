<script>
  import { getContext } from "svelte";
  import * as transaction from "../../src/transaction.ts";

  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  export let handle = null;
  export let identity = null;

  // This will be user-customizable in the future.
  export let transactionFee = null;

  export let onNextStep = null;
  export let onPreviousStep = null;

  const { registrationFee } = getContext("session");

  const tx = {
    fee: transactionFee,
    registrationFee: registrationFee.user,
    messages: [
      {
        type: transaction.MessageType.UserRegistration,
        handle,
        id: identity.id,
      },
    ],
  };

  // TODO(nuno): call transaction.getPayer(tx.msg, session) then build payer
  const payer = async () =>
    await transaction.getPayer(tx.messages[0], identity);
</script>

<Transaction transaction={tx} payer={payer()} />

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
