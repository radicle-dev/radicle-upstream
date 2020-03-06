<script>
  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  export let handle = null;

  export let onNextStep = null;
  export let onPreviousStep = null;

  const tx = {
    message: "Handle registration",
    stake: "Handle registration deposit",
    subject: {
      name: handle,
      kind: "user"
    },
    payer: {
      name: handle,
      kind: "user"
    }
  };
</script>

<Transaction {tx} />

<Flex style="margin-top: 32px;">
  <div slot="left">
    <Button
      disabled={false}
      on:click={onPreviousStep}
      variant="outline"
      style="margin-right: 24px">
      Back
    </Button>
  </div>

  <div slot="right">
    <Button disabled={false} on:click={onNextStep} variant="primary">
      Submit transaction
    </Button>
  </div>
</Flex>
