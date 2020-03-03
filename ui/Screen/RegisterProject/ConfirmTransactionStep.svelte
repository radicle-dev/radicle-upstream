<script>
  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  export let projectName = null;

  export let onNextStep = null;
  export let onPreviousStep = null;
</script>

<Transaction
  txMessage="Name registration"
  subjectName={projectName}
  stakeName="Project Registration Fee"
  payerName="cloudhead" />

<Flex style="margin-top: 48px;">
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
