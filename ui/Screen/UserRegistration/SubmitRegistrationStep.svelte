<script>
  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  export let handle = null;
  export let identity = null;

  export let onNextStep = null;
  export let onPreviousStep = null;

  const transaction = {
    message: "Handle registration",
    stake: "Handle registration deposit",
    subject: {
      name: handle,
      kind: "user",
      avatarFallback: identity.avatarFallback,
      imageUrl: identity.avatarUrl
    },
    payer: {
      name: handle,
      kind: "user",
      avatarFallback: identity.avatarFallback,
      imageUrl: identity.metadata.avatarUrl
    }
  };
</script>

<Transaction {transaction} />

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
