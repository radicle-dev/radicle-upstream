<script>
  import { createEventDispatcher } from "svelte";
  import { pop } from "svelte-spa-router";

  import * as transaction from "../../src/transaction.ts";
  import * as project from "../../src/project.ts";

  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  const dispatch = createEventDispatcher();

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

  const wallet = {
    name: registrarHandle,
    imageUrl: registrarImageUrl,
    avatarFallback: registrarAvatarFallback,
    variant: registrarVariant
  };

  const subject = {
    name: `${registrarHandle} / ${projectName}`,
    imageUrl: registrarImageUrl,
    avatarFallback: registrarAvatarFallback,
    variant: registrarVariant
  };

  const tx = {
    messages: [{ type: transaction.MessageType.ProjectRegistration }]
  };
</script>

<Transaction payer={wallet} {subject} transaction={tx} />

<Flex style="margin-top: 32px;" align="right">
  <Button
    dataCy="cancel-button"
    variant="transparent"
    on:click={() => dispatch('previous')}
    style="margin-right: 24px;">
    Back
  </Button>
  <Button dataCy="next-button" on:click={onSubmitTransaction} variant="primary">
    Submit transaction
  </Button>
</Flex>
