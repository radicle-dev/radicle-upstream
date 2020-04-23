<script>
  import * as identity from "../../src/identity.ts";
  import * as transaction from "../../src/transaction.ts";

  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  export let projectName = null;

  export let onNextStep = null;
  export let onPreviousStep = null;

  const tx = {
    messages: [
      {
        type: transaction.MessageType.ProjectRegistration,
        projectName,
        orgId: "monadic"
      }
    ]
  };
  const payer = transaction.formatPayer(identity.fallback);
  const subject = {
    name: projectName,
    kind: "project"
  };
</script>

<Transaction transaction={tx} {payer} {subject} />

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
