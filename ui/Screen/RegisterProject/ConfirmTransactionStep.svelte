<script>
  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  export let projectName = null;

  export let onNextStep = null;
  export let onPreviousStep = null;

  const transaction = {
    message: "Name registration",
    stake: "Project registration fee",
    subject: {
      name: projectName,
      kind: "project"
    },
    payer: {
      name: "cloudhead",
      kind: "user"
    }
  };
</script>

<Transaction {transaction} />

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
      Pay
    </Button>
  </div>
</Flex>
