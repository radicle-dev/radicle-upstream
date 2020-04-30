<script>
  import { createEventDispatcher } from "svelte";
  import { pop } from "svelte-spa-router";

  import * as transaction from "../../src/transaction.ts";
  import * as project from "../../src/project.ts";

  import { NavigationButtons, Transaction } from "../../DesignSystem/Component";

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

<NavigationButtons
  style={'margin-top: 32px;'}
  cancelLabel="Back"
  submitLabel="Submit transaction"
  on:cancel={() => {
    dispatch('previous');
  }}
  on:submit={onSubmitTransaction} />
