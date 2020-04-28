<script>
  import { pop } from "svelte-spa-router";

  import * as transaction from "../../src/transaction.ts";
  import * as project from "../../src/project.ts";

  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  export let projectId = null;
  export let registrarHandle = null;
  export let registrarImageUrl = null;
  export let registrarAvatarFallback = null;
  export let registrarVariant = null;

  export let projectName = null;

  const onSubmitTransaction = () => {
    project.register(registrarHandle, projectName, projectId);

    pop();
  };
</script>

<Transaction
  payer={{ name: registrarHandle, imageUrl: registrarImageUrl, avatarFallback: registrarAvatarFallback, variant: registrarVariant }}
  subject={{ name: 'subject' }}
  transaction={{ messages: [{ type: transaction.MessageType.ProjectRegistration }] }} />

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
