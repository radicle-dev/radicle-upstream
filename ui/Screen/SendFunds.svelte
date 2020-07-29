<script>
  import { getContext } from "svelte";
  import { pop } from "svelte-spa-router";
  import * as notification from "../src/notification.ts";
  import {
    TransferState,
    payerStore,
    recipientStore,
    amountStore,
    transfer,
  } from "../src/transfer.ts";

  import { Dropdown, ModalLayout, Rad } from "../DesignSystem/Component";
  import Row from "../DesignSystem/Component/Transaction/Row.svelte";
  import {
    Avatar,
    Button,
    Icon,
    Input,
    Text,
    Title,
  } from "../DesignSystem/Primitive";

  const { identity } = getContext("session");
  const { orgs } = getContext("session");

  // pre-populate inputs with store values
  let amount = $amountStore,
    recipient = $recipientStore,
    payer = $payerStore;

  const dropdownOptions = [];
  dropdownOptions.push({
    variant: "avatar",
    value: identity.metadata.handle,
    avatarProps: {
      variant: "circle",
      title: identity.metadata.handle,
      avatarFallback: identity.avatarFallback,
      imageUrl: identity.imageUrl,
    },
  });
  for (let i = 0; i < orgs.length; i++) {
    dropdownOptions.push({
      variant: "avatar",
      value: orgs[i].id,
      avatarProps: {
        variant: "square",
        title: orgs[i].id,
        avatarFallback: orgs[i].avatarFallback,
        imageUrl: null,
      },
    });
  }

  let state = TransferState.Preparation;

  const nextStep = () => {
    state = TransferState.Confirmation;
  };

  const previousStep = () => {
    state = TransferState.Preparation;
  };

  // TODO(nuno): use transaction fee defined by user once possible
  const transactionFee = 1;

  const onConfirmed = async () => {
    try {
      await transfer(
        $payerStore,
        parseInt($amountStore),
        $recipientStore,
        transactionFee
      );
    } catch (error) {
      notification.error(`Could not transfer funds: ${error.message}`);
    } finally {
      pop();
    }
  };

  // update each store whenever the input values change
  $: amountStore.set(amount);
  $: recipientStore.set(recipient);
  $: payerStore.set(payer);
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    flex-direction: column;
    margin: 92px 0 32px 0;
    width: 540px;
  }
  header {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    padding: 2rem;
    margin-bottom: 1.5rem;
    background-color: var(--color-foreground-level-1);
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.25rem;
  }
  .icon {
    height: 2.5rem;
    width: 2.5rem;
    border-radius: 1.25rem;
    background-color: var(--color-primary-level-1);
    display: flex;
    justify-content: center;
    align-items: center;
    margin-bottom: 1rem;
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    padding-top: 1.5rem;
  }

  .from-to {
    display: grid;
    grid-template-columns: 13rem 1.5rem 13rem;
    grid-column-gap: 1rem;
    margin-top: 1rem;
  }

  .from {
    display: flex;
    justify-content: flex-end;
  }
</style>

<ModalLayout dataCy="page">
  <div class="wrapper">
    {#if state === TransferState.Preparation}
      <header>
        <div class="icon">
          <Icon.ArrowUp style="fill: var(--color-primary)" />
        </div>
        <Title variant="big">Outgoing transfer</Title>
      </header>
      <Title
        style="color: var(--color-foreground-level-6); padding: 0 0.5rem 0.5rem
        0.5rem;">
        To
      </Title>
      <Input.Text
        bind:value={recipient}
        placeholder="Enter recipient address"
        style="flex: 1; padding-bottom: 0.5rem;" />
      <Title style="color: var(--color-foreground-level-6); padding: 0.5rem;">
        Amount
      </Title>
      <Input.Text
        placeholder="Enter the amount"
        bind:value={amount}
        showLeftItem
        style="flex: 1; padding-bottom: 0.5rem;">
        <div slot="left" style="display: flex;">
          <Icon.Currency
            size="big"
            style="fill: var(--color-foreground-level-6)" />
        </div>
      </Input.Text>
      <Title style="color: var(--color-foreground-level-6); padding: 0.5rem;">
        From
      </Title>
      <!-- TODO(julien): shouldn't identity.accountId be the same as fromAddress -->
      <Dropdown
        placeholder="Select wallet you want to use"
        bind:value={payer}
        options={dropdownOptions} />
      <div class="submit">
        <Button on:click={() => nextStep()}>Review transfer</Button>
      </div>
    {/if}
    {#if state === TransferState.Confirmation}
      <header>
        <div class="icon">
          <Icon.ArrowUp style="fill: var(--color-primary)" />
        </div>
        <Title variant="big">Outgoing transfer</Title>
        <div class="from-to">
          <div class="from">
            {#each dropdownOptions as option}
              {#if option.value === $payerStore}
                <Avatar
                  size="small"
                  title={option.avatarProps.title}
                  variant={option.avatarProps.variant}
                  avatarFallback={option.avatarProps.avatarFallback} />
              {/if}
            {/each}
          </div>
          <Icon.ArrowRight />
          <div class="to">
            <Title truncate style="color: var(--color-foreground-level-6);">
              {$recipientStore}
            </Title>
          </div>
        </div>
      </header>
      <Row variant="top" style="">
        <div slot="left">
          <Text
            variant="regular"
            style="color:var(--color-foreground-level-6);">
            Amount
          </Text>
        </div>

        <div slot="right">
          <Rad rad={$amountStore} />
        </div>
      </Row>
      <Row variant="middle" style="">
        <div slot="left">
          <Text
            variant="regular"
            style="color:var(--color-foreground-level-6);">
            Transaction Fee
          </Text>
        </div>

        <div slot="right">
          <Rad rad={$amountStore} />
        </div>
      </Row>
      <Row
        variant="bottom"
        style="border: 1px solid var(--color-foreground-level-2);">
        <div slot="left">
          <Title
            variant="regular"
            style="color:var(--color-foreground-level-6);">
            Total
          </Title>
        </div>

        <div slot="right">
          <Rad rad={$amountStore} />
        </div>
      </Row>
      <Row style="margin-top: 1.5rem;">
        <div slot="left">
          <Text
            variant="regular"
            style="color:var(--color-foreground-level-6);">
            Funding source
          </Text>
        </div>

        <div slot="right">
          {#each dropdownOptions as option}
            {#if option.value === $payerStore}
              <Avatar
                size="small"
                title={option.avatarProps.title}
                variant={option.avatarProps.variant}
                avatarFallback={option.avatarProps.avatarFallback} />
            {/if}
          {/each}
        </div>
      </Row>
      <div class="submit">
        <Button variant="transparent" on:click={() => previousStep()}>
          back
        </Button>
        <Button style="margin-left: 1rem;" on:click={onConfirmed}>
          Confirm and send
        </Button>
      </div>
    {/if}
  </div>
</ModalLayout>
