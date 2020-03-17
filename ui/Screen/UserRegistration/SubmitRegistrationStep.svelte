<script>
  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  export let handle = null;
  export let avatarFallback = null;
  export let imageUrl = null;

  export let onNextStep = null;
  export let onPreviousStep = null;

  const tx = {
    message: "Handle registration",
    stake: "Handle registration deposit",
    subject: {
      name: handle,
      kind: "user",
      avatarFallback: avatarFallback,
      imageUrl: imageUrl
    },
    payer: {
      name: handle,
      kind: "user",
      avatarFallback: avatarFallback,
      imageUrl: imageUrl
    }
  };
</script>

<Transaction {tx} style="margin-top: 32px;" />

<Flex style="margin-top: 32px;" align="right">
  <Button
    disabled={false}
    on:click={onPreviousStep}
    variant="transparent"
    style="margin-right: 24px">
    Back
  </Button>
  <Button disabled={false} on:click={onNextStep} variant="primary">
    Submit transaction
  </Button>
</Flex>
